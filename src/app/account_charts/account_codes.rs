use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GlAccountCode {
    // ─── ASSETS (1xxx) ───────────────────────────────────────────────
    #[serde(rename = "CASH_IN_HAND")]
    CashInHand,
    #[serde(rename = "CASH_AT_CENTRAL_BANK")]
    CashAtCentralBank,
    #[serde(rename = "CASH_AT_OTHER_BANKS")]
    CashAtOtherBanks,
    #[serde(rename = "LOANS_AND_ADVANCES")]
    LoansAndAdvances,
    #[serde(rename = "LOANS_TO_CUSTOMERS")]
    LoansToCustomers,
    #[serde(rename = "LOANS_TO_BANKS")]
    LoansToBanks,
    #[serde(rename = "INTEREST_RECEIVABLE")]
    InterestReceivable,
    #[serde(rename = "FIXED_ASSETS")]
    FixedAssets,
    #[serde(rename = "ACCUMULATED_DEPRECIATION")]
    AccumulatedDepreciation,
    #[serde(rename = "PREPAID_EXPENSES")]
    PrepaidExpenses,
    #[serde(rename = "INVESTMENT_SECURITIES")]
    InvestmentSecurities,

    // ─── LIABILITIES (2xxx) ──────────────────────────────────────────
    #[serde(rename = "CUSTOMER_DEPOSITS")]
    CustomerDeposits,
    #[serde(rename = "CUSTOMER_SAVINGS")]
    CustomerSavings,
    #[serde(rename = "CUSTOMER_CURRENT")]
    CustomerCurrent,
    #[serde(rename = "CUSTOMER_FIXED_DEPOSITS")]
    CustomerFixedDeposits,
    #[serde(rename = "CUSTOMER_DEPOSITS_SAVINGS")]
    CustomerDepositsSavings,
    #[serde(rename = "CUSTOMER_DEPOSITS_CURRENT")]
    CustomerDepositsCurrent,
    #[serde(rename = "INTERBANK_BORROWINGS")]
    InterbankBorrowings,
    #[serde(rename = "BORROWINGS_FROM_CENTRAL_BANK")]
    BorrowingsFromCentralBank,
    #[serde(rename = "INTEREST_PAYABLE")]
    InterestPayable,
    #[serde(rename = "ACCOUNTS_PAYABLE")]
    AccountsPayable,
    #[serde(rename = "TAX_PAYABLE")]
    TaxPayable,
    #[serde(rename = "ACCRUED_EXPENSES")]
    AccruedExpenses,
    #[serde(rename = "PROVISIONS")]
    Provisions,
    #[serde(rename = "LOAN_LOSS_RESERVE")]
    LoanLossReserve,

    // ─── EQUITY (3xxx) ───────────────────────────────────────────────
    #[serde(rename = "SHARE_CAPITAL")]
    ShareCapital,
    #[serde(rename = "PREFERRED_STOCK")]
    PreferredStock,
    #[serde(rename = "SHARE_PREMIUM")]
    SharePremium,
    #[serde(rename = "RETAINED_EARNINGS")]
    RetainedEarnings,
    #[serde(rename = "STATUTORY_RESERVE")]
    StatutoryReserve,
    #[serde(rename = "GENERAL_RESERVE")]
    GeneralReserve,
    #[serde(rename = "REVALUATION_RESERVE")]
    RevaluationReserve,

    // ─── INCOME (4xxx) ───────────────────────────────────────────────
    #[serde(rename = "INTEREST_INCOME_LOANS")]
    InterestIncomeLoans,
    #[serde(rename = "INTEREST_INCOME_INVESTMENTS")]
    InterestIncomeInvestments,
    #[serde(rename = "INTEREST_INCOME_INTERBANK")]
    InterestIncomeInterbank,
    #[serde(rename = "FEE_INCOME")]
    FeeIncome,
    #[serde(rename = "FEE_INCOME_ACCOUNT_MAINTENANCE")]
    FeeIncomeAccountMaintenance,
    #[serde(rename = "FEE_INCOME_TRANSACTION")]
    FeeIncomeTransaction,
    #[serde(rename = "FEE_INCOME_LOAN_ORIGINATION")]
    FeeIncomeLoanOrigination,
    #[serde(rename = "FOREX_INCOME")]
    ForexIncome,
    #[serde(rename = "TRADING_INCOME")]
    TradingIncome,
    #[serde(rename = "OTHER_INCOME")]
    OtherIncome,
    #[serde(rename = "RECOVERIES")]
    Recoveries,

    // ─── EXPENSES (5xxx) ─────────────────────────────────────────────
    #[serde(rename = "INTEREST_EXPENSE_DEPOSITS")]
    InterestExpenseDeposits,
    #[serde(rename = "INTEREST_EXPENSE_BORROWINGS")]
    InterestExpenseBorrowings,
    #[serde(rename = "SALARIES_AND_WAGES")]
    SalariesAndWages,
    #[serde(rename = "EMPLOYEE_BENEFITS")]
    EmployeeBenefits,
    #[serde(rename = "RENT_EXPENSE")]
    RentExpense,
    #[serde(rename = "UTILITIES_EXPENSE")]
    UtilitiesExpense,
    #[serde(rename = "DEPRECIATION_EXPENSE")]
    DepreciationExpense,
    #[serde(rename = "AMORTIZATION_EXPENSE")]
    AmortizationExpense,
    #[serde(rename = "LOAN_LOSS_PROVISION")]
    LoanLossProvision,
    #[serde(rename = "PROFESSIONAL_FEES")]
    ProfessionalFees,
    #[serde(rename = "MARKETING_EXPENSE")]
    MarketingExpense,
    #[serde(rename = "TECHNOLOGY_EXPENSE")]
    TechnologyExpense,
    #[serde(rename = "BANK_CHARGES")]
    BankCharges,
    #[serde(rename = "INSURANCE_EXPENSE")]
    InsuranceExpense,
    #[serde(rename = "OTHER_OPERATING_EXPENSE")]
    OtherOperatingExpense,

    // ─── CONTRA / OFF-BALANCE (6xxx–9xxx) ────────────────────────────
    #[serde(rename = "SUSPENSE_ACCOUNT")]
    SuspenseAccount,
    #[serde(rename = "INTERNAL_CLEARING")]
    InternalClearing,
    #[serde(rename = "CUSTOMER_DEPOSITS_CONTRA")]
    CustomerDepositsContra,
    #[serde(rename = "UNCLAIMED_DEPOSITS")]
    UnclaimedDeposits,
    #[serde(rename = "COMMITMENTS")]
    Commitments,
    #[serde(rename = "GUARANTEES_ISSUED")]
    GuaranteesIssued,
    #[serde(rename = "LETTERS_OF_CREDIT")]
    LettersOfCredit,
}

impl GlAccountCode {
    /// Returns the canonical DB string for this code.
    pub const fn as_str(&self) -> &'static str {
        match self {
            GlAccountCode::CashInHand => "CASH_IN_HAND",
            GlAccountCode::CashAtCentralBank => "CASH_AT_CENTRAL_BANK",
            GlAccountCode::CashAtOtherBanks => "CASH_AT_OTHER_BANKS",
            GlAccountCode::LoansAndAdvances => "LOANS_AND_ADVANCES",
            GlAccountCode::LoansToCustomers => "LOANS_TO_CUSTOMERS",
            GlAccountCode::LoansToBanks => "LOANS_TO_BANKS",
            GlAccountCode::InterestReceivable => "INTEREST_RECEIVABLE",
            GlAccountCode::FixedAssets => "FIXED_ASSETS",
            GlAccountCode::AccumulatedDepreciation => "ACCUMULATED_DEPRECIATION",
            GlAccountCode::PrepaidExpenses => "PREPAID_EXPENSES",
            GlAccountCode::InvestmentSecurities => "INVESTMENT_SECURITIES",
            GlAccountCode::CustomerDeposits => "CUSTOMER_DEPOSITS",
            GlAccountCode::CustomerSavings => "CUSTOMER_SAVINGS",
            GlAccountCode::CustomerCurrent => "CUSTOMER_CURRENT",
            GlAccountCode::CustomerFixedDeposits => "CUSTOMER_FIXED_DEPOSITS",
            GlAccountCode::CustomerDepositsSavings => "CUSTOMER_DEPOSITS_SAVINGS",
            GlAccountCode::CustomerDepositsCurrent => "CUSTOMER_DEPOSITS_CURRENT",
            GlAccountCode::InterbankBorrowings => "INTERBANK_BORROWINGS",
            GlAccountCode::BorrowingsFromCentralBank => "BORROWINGS_FROM_CENTRAL_BANK",
            GlAccountCode::InterestPayable => "INTEREST_PAYABLE",
            GlAccountCode::AccountsPayable => "ACCOUNTS_PAYABLE",
            GlAccountCode::TaxPayable => "TAX_PAYABLE",
            GlAccountCode::AccruedExpenses => "ACCRUED_EXPENSES",
            GlAccountCode::Provisions => "PROVISIONS",
            GlAccountCode::LoanLossReserve => "LOAN_LOSS_RESERVE",
            GlAccountCode::ShareCapital => "SHARE_CAPITAL",
            GlAccountCode::PreferredStock => "PREFERRED_STOCK",
            GlAccountCode::SharePremium => "SHARE_PREMIUM",
            GlAccountCode::RetainedEarnings => "RETAINED_EARNINGS",
            GlAccountCode::StatutoryReserve => "STATUTORY_RESERVE",
            GlAccountCode::GeneralReserve => "GENERAL_RESERVE",
            GlAccountCode::RevaluationReserve => "REVALUATION_RESERVE",
            GlAccountCode::InterestIncomeLoans => "INTEREST_INCOME_LOANS",
            GlAccountCode::InterestIncomeInvestments => "INTEREST_INCOME_INVESTMENTS",
            GlAccountCode::InterestIncomeInterbank => "INTEREST_INCOME_INTERBANK",
            GlAccountCode::FeeIncome => "FEE_INCOME",
            GlAccountCode::FeeIncomeAccountMaintenance => "FEE_INCOME_ACCOUNT_MAINTENANCE",
            GlAccountCode::FeeIncomeTransaction => "FEE_INCOME_TRANSACTION",
            GlAccountCode::FeeIncomeLoanOrigination => "FEE_INCOME_LOAN_ORIGINATION",
            GlAccountCode::ForexIncome => "FOREX_INCOME",
            GlAccountCode::TradingIncome => "TRADING_INCOME",
            GlAccountCode::OtherIncome => "OTHER_INCOME",
            GlAccountCode::Recoveries => "RECOVERIES",
            GlAccountCode::InterestExpenseDeposits => "INTEREST_EXPENSE_DEPOSITS",
            GlAccountCode::InterestExpenseBorrowings => "INTEREST_EXPENSE_BORROWINGS",
            GlAccountCode::SalariesAndWages => "SALARIES_AND_WAGES",
            GlAccountCode::EmployeeBenefits => "EMPLOYEE_BENEFITS",
            GlAccountCode::RentExpense => "RENT_EXPENSE",
            GlAccountCode::UtilitiesExpense => "UTILITIES_EXPENSE",
            GlAccountCode::DepreciationExpense => "DEPRECIATION_EXPENSE",
            GlAccountCode::AmortizationExpense => "AMORTIZATION_EXPENSE",
            GlAccountCode::LoanLossProvision => "LOAN_LOSS_PROVISION",
            GlAccountCode::ProfessionalFees => "PROFESSIONAL_FEES",
            GlAccountCode::MarketingExpense => "MARKETING_EXPENSE",
            GlAccountCode::TechnologyExpense => "TECHNOLOGY_EXPENSE",
            GlAccountCode::BankCharges => "BANK_CHARGES",
            GlAccountCode::InsuranceExpense => "INSURANCE_EXPENSE",
            GlAccountCode::OtherOperatingExpense => "OTHER_OPERATING_EXPENSE",
            GlAccountCode::SuspenseAccount => "SUSPENSE_ACCOUNT",
            GlAccountCode::InternalClearing => "INTERNAL_CLEARING",
            GlAccountCode::CustomerDepositsContra => "CUSTOMER_DEPOSITS_CONTRA",
            GlAccountCode::UnclaimedDeposits => "UNCLAIMED_DEPOSITS",
            GlAccountCode::Commitments => "COMMITMENTS",
            GlAccountCode::GuaranteesIssued => "GUARANTEES_ISSUED",
            GlAccountCode::LettersOfCredit => "LETTERS_OF_CREDIT",
        }
    }
}

impl fmt::Display for GlAccountCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

// impl AsRef<str> for GlAccountCode {
//     fn as_ref(&self) -> &str {
//         self.as_str()
//     }
// }

// impl From<GlAccountCode> for String {
//     fn from(code: GlAccountCode) -> Self {
//         code.as_str().to_string()
//     }
// }
