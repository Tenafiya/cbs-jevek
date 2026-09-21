use crate::utils::models::{CashParams, ChequeParams};
use rust_decimal::Decimal;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AmountParamError {
    #[error("amount must be greater than zero")]
    NonPositiveAmount,

    #[error("cash breakdown total ({cash_total}) does not match amount ({amount})")]
    CashBreakdownMismatch {
        amount: Decimal,
        cash_total: Decimal,
    },

    #[error("cheques total ({cheque_total}) does not match amount ({amount})")]
    ChequesMismatch {
        amount: Decimal,
        cheque_total: Decimal,
    },

    #[error("cash breakdown + cheques total ({combined}) does not match amount ({amount})")]
    CombinedMismatch {
        amount: Decimal,
        cash_total: Decimal,
        cheque_total: Decimal,
        combined: Decimal,
    },

    #[error("no payment instrument provided: cash breakdown and cheques are both empty")]
    NoInstruments,

    #[error("arithmetic overflow while summing instruments")]
    Overflow,
}

pub fn validate_amount_param(
    amount: Decimal,
    cash_breakdown: Option<&[CashParams]>,
    cheques: Option<&[ChequeParams]>,
) -> Result<(), AmountParamError> {
    if amount <= Decimal::ZERO {
        return Err(AmountParamError::NonPositiveAmount);
    }

    let cash_total = match cash_breakdown {
        Some(cash) if !cash.is_empty() => sum_cash(cash)?,
        _ => Decimal::ZERO,
    };

    let cheque_total = match cheques {
        Some(cheques) if !cheques.is_empty() => sum_cheques(cheques)?,
        _ => Decimal::ZERO,
    };

    let has_cash = cash_breakdown.map_or(false, |c| !c.is_empty());
    let has_cheques = cheques.map_or(false, |c| !c.is_empty());

    match (has_cash, has_cheques) {
        (false, false) => return Err(AmountParamError::NoInstruments),

        // Only cash provided: amount must equal cash total
        (true, false) => {
            if amount != cash_total {
                return Err(AmountParamError::CashBreakdownMismatch { amount, cash_total });
            }
        }

        // Only cheques provided: amount must equal cheques total
        (false, true) => {
            if amount != cheque_total {
                return Err(AmountParamError::ChequesMismatch {
                    amount,
                    cheque_total,
                });
            }
        }

        // Both provided: amount must equal cash + cheques
        (true, true) => {
            let combined = cash_total
                .checked_add(cheque_total)
                .ok_or(AmountParamError::Overflow)?;

            if amount != combined {
                return Err(AmountParamError::CombinedMismatch {
                    amount,
                    cash_total,
                    cheque_total,
                    combined,
                });
            }
        }
    }

    Ok(())
}

fn sum_cash(cash: &[CashParams]) -> Result<Decimal, AmountParamError> {
    cash.iter().try_fold(Decimal::ZERO, |acc, c| {
        let line = c
            .denomination
            .checked_mul(Decimal::from(c.quantity))
            .ok_or(AmountParamError::Overflow)?;

        acc.checked_add(line).ok_or(AmountParamError::Overflow)
    })
}

fn sum_cheques(cheques: &[ChequeParams]) -> Result<Decimal, AmountParamError> {
    cheques.iter().try_fold(Decimal::ZERO, |acc, c| {
        acc.checked_add(c.amount).ok_or(AmountParamError::Overflow)
    })
}
