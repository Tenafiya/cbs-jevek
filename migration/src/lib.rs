pub use sea_orm_migration::prelude::*;

mod m20251204_103935_create_countries;
mod m20251204_112805_create_institutions;
mod m20251204_150208_create_branches;
mod m20251204_151411_create_chart_of_accounts;
mod m20251204_152312_create_customers;
mod m20251204_155243_create_customer_identifications;
mod m20251204_160520_create_customer_address_verifications;
mod m20251204_161342_create_customer_blacklist_records;
mod m20251205_150707_create_account_categories;
mod m20251205_151223_create_account_types;
mod m20251205_154503_create_accounts;
mod m20251205_160629_create_account_balances;
mod m20251205_162130_create_account_links;
mod m20251205_163328_create_account_limits;
mod m20251205_164546_create_fixed_deposit_accounts;
mod m20251205_165925_create_transaction_channels;
mod m20251205_191200_create_transaction_limits;
mod m20251205_193221_create_transactions;
mod m20251205_200840_create_standing_orders;
mod m20251205_202808_create_transaction_reversals;
mod m20251205_204238_create_transaction_disputes;
mod m20251205_205817_create_loan_product_types;
mod m20251205_210647_create_loan_products;
mod m20251206_143556_create_loan_applications;
mod m20251206_150156_create_loan_application_status_history;
mod m20251206_150936_create_loans;
mod m20251206_153829_create_loan_repayment_schedules;
mod m20251206_165752_create_loan_repayments;
mod m20251206_171201_create_loan_collateral;
mod m20251206_172353_create_loan_collateral_valuations;
mod m20251206_172956_create_loan_guarantors;
mod m20251206_174124_create_loan_penalties;
mod m20251206_175121_create_loan_rescheduling;
mod m20251206_191556_create_loan_write_offs;
mod m20251206_192322_create_loan_provisioning;
mod m20251206_193123_create_savings_products;
mod m20251207_103023_create_saving_goals;
mod m20251207_131906_create_contribution_cycles;
mod m20251207_141343_create_contributions;
mod m20251207_182150_create_group_saving_members;
mod m20251207_183348_create_wallet_providers;
mod m20251207_184227_create_wallets;
mod m20251207_185206_create_wallet_transactions;
mod m20251207_195049_create_wallet_reconciliations;
mod m20251207_212120_create_cards;
mod m20251207_214445_create_card_transactions;
mod m20251208_084615_create_card_limits;
mod m20251208_090443_create_card_disputes;
mod m20251208_093551_create_tellers;
mod m20251208_101414_create_teller_cash_drawers;
mod m20251208_151022_create_vaults;
mod m20251208_152111_create_cash_transfers;
mod m20251208_153457_create_teller_reconciliations;
mod m20251208_154224_create_agents;
mod m20251208_161400_create_agent_kyc_docs;
mod m20251208_162111_create_agent_wallets;
mod m20251208_163720_create_agent_commissions;
mod m20251209_191256_create_agent_transactions;
mod m20251209_195653_create_agent_settlements;
mod m20251209_200508_create_agent_performance;
mod m20251209_200814_create_agent_audits;
mod m20251209_204000_create_staff_commissions;
mod m20251210_160801_create_staff_attendance;
mod m20251210_161034_create_staff_tasks;
mod m20251210_161623_create_field_officer_routes;
mod m20251210_162142_create_field_officer_visits;
mod m20251210_162815_create_product_change_workflows;
mod m20251210_165057_create_product_visibility_rules;
mod m20251210_165956_create_fee_types;
mod m20251210_172518_create_fee_applications;
mod m20251210_173128_create_fee_waiver_workflows;
mod m20251210_173510_create_commission_payouts;
mod m20251210_174118_create_amls;
mod m20251210_175201_create_suspicious_transaction_reports;
mod m20251210_175839_create_currency_transaction_reports;
mod m20251210_180706_create_regulatory_reports;
mod m20251210_181206_create_audit_trails;
mod m20251210_182035_create_maker_checker_workflows;
mod m20251210_182628_create_data_retention_rules;
mod m20251210_183037_create_tax_withholding;
mod m20251210_185433_create_gl_daily_balances;
mod m20251210_190018_create_gl_postings;
mod m20251210_191157_create_gl_reversals;
mod m20251210_203914_create_accruals_and_provisions;
mod m20251210_204505_create_financial_statements;
mod m20251210_205002_create_ledger_lock_periods;
mod m20251210_205331_create_report_schedules;
mod m20251210_205945_create_generated_reports;
mod m20251212_191919_create_report_analytics_caches;
mod m20251212_192129_create_notification_templates;
mod m20251212_192637_create_notification_queues;
mod m20251212_193027_create_notification_preferences;
mod m20251212_193227_create_support_tickets;
mod m20251212_193841_create_support_interactions;
mod m20251212_194235_create_refund_workflows;
mod m20251212_194802_create_chargebacks;
mod m20251212_195219_create_sla_configurations;
mod m20251212_195551_create_integration_providers;
mod m20251212_200017_create_integration_apikeys;
mod m20251212_200256_create_integration_webhooks;
mod m20251212_200614_create_integration_webhook_deliveries;
mod m20251212_203308_create_integration_logs;
mod m20251212_203641_create_credit_bureau_reports;
mod m20251212_204011_create_kyc_provider_checks;
mod m20251212_204334_create_customer_sessions;
mod m20251212_204801_create_customer_app_preferences;
mod m20251212_205056_create_customer_education_contents;
mod m20251212_205422_create_super_admins;
mod m20251212_205650_create_admin_audit_logs;
mod m20251212_210040_create_health_n_maintenance;
mod m20251212_210457_create_data_backups;
mod m20251212_210901_create_regulatory_reporting_exports;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20251204_103935_create_countries::Migration),
            Box::new(m20251204_112805_create_institutions::Migration),
            Box::new(m20251204_150208_create_branches::Migration),
            Box::new(m20251204_151411_create_chart_of_accounts::Migration),
            Box::new(m20251204_152312_create_customers::Migration),
            Box::new(m20251204_155243_create_customer_identifications::Migration),
            Box::new(m20251204_160520_create_customer_address_verifications::Migration),
            Box::new(m20251204_161342_create_customer_blacklist_records::Migration),
            Box::new(m20251205_150707_create_account_categories::Migration),
            Box::new(m20251205_151223_create_account_types::Migration),
            Box::new(m20251205_154503_create_accounts::Migration),
            Box::new(m20251205_160629_create_account_balances::Migration),
            Box::new(m20251205_162130_create_account_links::Migration),
            Box::new(m20251205_163328_create_account_limits::Migration),
            Box::new(m20251205_164546_create_fixed_deposit_accounts::Migration),
            Box::new(m20251205_165925_create_transaction_channels::Migration),
            Box::new(m20251205_191200_create_transaction_limits::Migration),
            Box::new(m20251205_193221_create_transactions::Migration),
            Box::new(m20251205_200840_create_standing_orders::Migration),
            Box::new(m20251205_202808_create_transaction_reversals::Migration),
            Box::new(m20251205_204238_create_transaction_disputes::Migration),
            Box::new(m20251205_205817_create_loan_product_types::Migration),
            Box::new(m20251205_210647_create_loan_products::Migration),
            Box::new(m20251206_143556_create_loan_applications::Migration),
            Box::new(m20251206_150156_create_loan_application_status_history::Migration),
            Box::new(m20251206_150936_create_loans::Migration),
            Box::new(m20251206_153829_create_loan_repayment_schedules::Migration),
            Box::new(m20251206_165752_create_loan_repayments::Migration),
            Box::new(m20251206_171201_create_loan_collateral::Migration),
            Box::new(m20251206_172353_create_loan_collateral_valuations::Migration),
            Box::new(m20251206_172956_create_loan_guarantors::Migration),
            Box::new(m20251206_174124_create_loan_penalties::Migration),
            Box::new(m20251206_175121_create_loan_rescheduling::Migration),
            Box::new(m20251206_191556_create_loan_write_offs::Migration),
            Box::new(m20251206_192322_create_loan_provisioning::Migration),
            Box::new(m20251206_193123_create_savings_products::Migration),
            Box::new(m20251207_103023_create_saving_goals::Migration),
            Box::new(m20251207_131906_create_contribution_cycles::Migration),
            Box::new(m20251207_141343_create_contributions::Migration),
            Box::new(m20251207_182150_create_group_saving_members::Migration),
            Box::new(m20251207_183348_create_wallet_providers::Migration),
            Box::new(m20251207_184227_create_wallets::Migration),
            Box::new(m20251207_185206_create_wallet_transactions::Migration),
            Box::new(m20251207_195049_create_wallet_reconciliations::Migration),
            Box::new(m20251207_212120_create_cards::Migration),
            Box::new(m20251207_214445_create_card_transactions::Migration),
            Box::new(m20251208_084615_create_card_limits::Migration),
            Box::new(m20251208_090443_create_card_disputes::Migration),
            Box::new(m20251208_093551_create_tellers::Migration),
            Box::new(m20251208_101414_create_teller_cash_drawers::Migration),
            Box::new(m20251208_151022_create_vaults::Migration),
            Box::new(m20251208_152111_create_cash_transfers::Migration),
            Box::new(m20251208_153457_create_teller_reconciliations::Migration),
            Box::new(m20251208_154224_create_agents::Migration),
            Box::new(m20251208_161400_create_agent_kyc_docs::Migration),
            Box::new(m20251208_162111_create_agent_wallets::Migration),
            Box::new(m20251208_163720_create_agent_commissions::Migration),
            Box::new(m20251209_191256_create_agent_transactions::Migration),
            Box::new(m20251209_195653_create_agent_settlements::Migration),
            Box::new(m20251209_200508_create_agent_performance::Migration),
            Box::new(m20251209_200814_create_agent_audits::Migration),
            Box::new(m20251209_204000_create_staff_commissions::Migration),
            Box::new(m20251210_160801_create_staff_attendance::Migration),
            Box::new(m20251210_161034_create_staff_tasks::Migration),
            Box::new(m20251210_161623_create_field_officer_routes::Migration),
            Box::new(m20251210_162142_create_field_officer_visits::Migration),
            Box::new(m20251210_162815_create_product_change_workflows::Migration),
            Box::new(m20251210_165057_create_product_visibility_rules::Migration),
            Box::new(m20251210_165956_create_fee_types::Migration),
            Box::new(m20251210_172518_create_fee_applications::Migration),
            Box::new(m20251210_173128_create_fee_waiver_workflows::Migration),
            Box::new(m20251210_173510_create_commission_payouts::Migration),
            Box::new(m20251210_174118_create_amls::Migration),
            Box::new(m20251210_175201_create_suspicious_transaction_reports::Migration),
            Box::new(m20251210_175839_create_currency_transaction_reports::Migration),
            Box::new(m20251210_180706_create_regulatory_reports::Migration),
            Box::new(m20251210_181206_create_audit_trails::Migration),
            Box::new(m20251210_182035_create_maker_checker_workflows::Migration),
            Box::new(m20251210_182628_create_data_retention_rules::Migration),
            Box::new(m20251210_183037_create_tax_withholding::Migration),
            Box::new(m20251210_185433_create_gl_daily_balances::Migration),
            Box::new(m20251210_190018_create_gl_postings::Migration),
            Box::new(m20251210_191157_create_gl_reversals::Migration),
            Box::new(m20251210_203914_create_accruals_and_provisions::Migration),
            Box::new(m20251210_204505_create_financial_statements::Migration),
            Box::new(m20251210_205002_create_ledger_lock_periods::Migration),
            Box::new(m20251210_205331_create_report_schedules::Migration),
            Box::new(m20251210_205945_create_generated_reports::Migration),
            Box::new(m20251212_191919_create_report_analytics_caches::Migration),
            Box::new(m20251212_192129_create_notification_templates::Migration),
            Box::new(m20251212_192637_create_notification_queues::Migration),
            Box::new(m20251212_193027_create_notification_preferences::Migration),
            Box::new(m20251212_193227_create_support_tickets::Migration),
            Box::new(m20251212_193841_create_support_interactions::Migration),
            Box::new(m20251212_194235_create_refund_workflows::Migration),
            Box::new(m20251212_194802_create_chargebacks::Migration),
            Box::new(m20251212_195219_create_sla_configurations::Migration),
            Box::new(m20251212_195551_create_integration_providers::Migration),
            Box::new(m20251212_200017_create_integration_apikeys::Migration),
            Box::new(m20251212_200256_create_integration_webhooks::Migration),
            Box::new(m20251212_200614_create_integration_webhook_deliveries::Migration),
            Box::new(m20251212_203308_create_integration_logs::Migration),
            Box::new(m20251212_203641_create_credit_bureau_reports::Migration),
            Box::new(m20251212_204011_create_kyc_provider_checks::Migration),
            Box::new(m20251212_204334_create_customer_sessions::Migration),
            Box::new(m20251212_204801_create_customer_app_preferences::Migration),
            Box::new(m20251212_205056_create_customer_education_contents::Migration),
            Box::new(m20251212_205422_create_super_admins::Migration),
            Box::new(m20251212_205650_create_admin_audit_logs::Migration),
            Box::new(m20251212_210040_create_health_n_maintenance::Migration),
            Box::new(m20251212_210457_create_data_backups::Migration),
            Box::new(m20251212_210901_create_regulatory_reporting_exports::Migration),
        ]
    }
}
