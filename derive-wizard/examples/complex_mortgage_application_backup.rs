//! Complex Mortgage Loan Application Form
//!
//! This is a comprehensive example showcasing a real-world complex form based on
//! mortgage applications for foreign nationals and visa holders. It demonstrates
//! deep nesting, multiple enum variants, conditional logic, and extensive validation.
//!
//! Based on mortgage application requirements for visa holders in 2026.

use derive_wizard::Wizard;

#[derive(Wizard, Debug)]
#[prelude(
    "MORTGAGE LOAN APPLICATION - Complete all sections accurately. Processing time: 43-47 days."
)]
#[epilogue(
    "I certify that all information provided is true and accurate. I understand that misrepresentation may result in loan denial or criminal prosecution."
)]
struct MortgageLoanApplication {
    // Section 1: Applicant Information
    #[prompt("PRIMARY BORROWER INFORMATION")]
    primary_borrower: BorrowerInfo,

    #[prompt("Do you have a co-borrower?")]
    has_co_borrower: bool,

    #[prompt("CO-BORROWER INFORMATION")]
    co_borrower: Option<BorrowerInfo>,

    // Section 2: Property Information
    #[prompt("PROPERTY INFORMATION")]
    property: PropertyInfo,

    // Section 3: Loan Details
    #[prompt("LOAN DETAILS")]
    loan_details: LoanDetails,

    // Section 4: Financial Information
    #[prompt("FINANCIAL INFORMATION")]
    financial_info: FinancialInformation,

    // Section 5: Employment & Income
    #[prompt("EMPLOYMENT AND INCOME")]
    employment: EmploymentHistory,

    // Section 6: Assets
    #[prompt("ASSETS")]
    assets: Assets,

    // Section 7: Liabilities
    #[prompt("LIABILITIES")]
    liabilities: Liabilities,

    // Section 8: Declarations
    #[prompt("DECLARATIONS")]
    declarations: Declarations,
}

#[derive(Wizard, Debug)]
struct BorrowerInfo {
    // Personal Details
    #[prompt("Full Legal Name (as appears on passport)")]
    full_name: String,

    #[prompt("Date of Birth (MM/DD/YYYY)")]
    date_of_birth: String,

    #[prompt("Social Security Number or ITIN")]
    ssn_or_itin: String,

    #[prompt("Citizenship Status")]
    citizenship: CitizenshipStatus,

    // Contact Information
    #[prompt("Current Address - Street")]
    current_street: String,

    #[prompt("City")]
    current_city: String,

    #[prompt("State")]
    current_state: String,

    #[prompt("ZIP Code")]
    current_zip: String,

    #[prompt("Years at current address")]
    #[min(0)]
    years_at_current: i64,

    #[prompt("Months at current address")]
    #[min(0)]
    #[max(11)]
    months_at_current: i64,

    #[prompt("Email Address")]
    email: String,

    #[prompt("Primary Phone Number")]
    phone: String,

    #[prompt("Marital Status")]
    marital_status: MaritalStatus,

    #[prompt("Number of Dependents")]
    #[min(0)]
    dependents: i64,
}

#[derive(Wizard, Debug)]
enum CitizenshipStatus {
    #[prompt("U.S. Citizen")]
    USCitizen,

    #[prompt("Permanent Resident (Green Card Holder)")]
    PermanentResident {
        #[prompt("Green Card Number")]
        green_card_number: String,

        #[prompt("Expiration Date")]
        expiration_date: String,
    },

    #[prompt("Non-Permanent Resident (Visa Holder)")]
    VisaHolder {
        #[prompt("Visa Type")]
        visa_type: VisaType,

        #[prompt("Passport Number")]
        passport_number: String,

        #[prompt("Passport Country")]
        passport_country: String,

        #[prompt("Visa Expiration Date")]
        visa_expiration: String,

        #[prompt("Years remaining on visa")]
        #[min(0)]
        years_remaining: i64,
    },

    #[prompt("Foreign National (No U.S. Residency)")]
    ForeignNational {
        #[prompt("Country of Residence")]
        country: String,

        #[prompt("Passport Number")]
        passport_number: String,
    },
}

#[derive(Wizard, Debug)]
enum VisaType {
    #[prompt("H-1B (Specialty Occupation Workers)")]
    H1B,

    #[prompt("L-1 (Intracompany Transferee)")]
    L1,

    #[prompt("E-2 (Treaty Investor)")]
    E2,

    #[prompt("O-1 (Extraordinary Ability)")]
    O1,

    #[prompt("F-1 (Student) with OPT")]
    F1,

    #[prompt("TN (NAFTA Professional)")]
    TN,

    Other,
}

#[derive(Wizard, Debug)]
enum MaritalStatus {
    Single,
    Married,
    Separated,
    Divorced,
    Widowed,
}

#[derive(Wizard, Debug)]
struct PropertyInfo {
    #[prompt("Property Address - Street")]
    street: String,

    #[prompt("City")]
    city: String,

    #[prompt("State")]
    state: String,

    #[prompt("ZIP Code")]
    zip: String,

    #[prompt("Property Type")]
    property_type: PropertyType,

    #[prompt("Number of Units")]
    #[min(1)]
    #[max(4)]
    units: i64,

    #[prompt("Year Built")]
    #[min(1800)]
    #[max(2026)]
    year_built: i64,

    #[prompt("Estimated Property Value")]
    #[min(50000)]
    estimated_value: i64,

    #[prompt("Occupancy Type")]
    occupancy: OccupancyType,
}

#[derive(Wizard, Debug)]
enum PropertyType {
    #[prompt("Single Family Residence")]
    SingleFamily,

    #[prompt("Condominium")]
    Condominium,

    #[prompt("Townhouse")]
    Townhouse,

    #[prompt("Multi-Family (2-4 units)")]
    MultiFamily,

    #[prompt("Cooperative")]
    Cooperative,
}

#[derive(Wizard, Debug)]
enum OccupancyType {
    #[prompt("Primary Residence")]
    PrimaryResidence,

    #[prompt("Second Home")]
    SecondHome,

    #[prompt("Investment Property")]
    Investment,
}

#[derive(Wizard, Debug)]
struct LoanDetails {
    #[prompt("Loan Amount Requested")]
    #[min(50000)]
    loan_amount: i64,

    #[prompt("Loan Purpose")]
    loan_purpose: LoanPurpose,

    #[prompt("Loan Term (years)")]
    loan_term: LoanTerm,

    #[prompt("Down Payment Amount")]
    #[min(0)]
    down_payment: i64,

    #[prompt("Source of Down Payment")]
    down_payment_source: DownPaymentSource,
}

#[derive(Wizard, Debug)]
enum LoanPurpose {
    #[prompt("Purchase")]
    Purchase,

    #[prompt("Refinance - Rate and Term")]
    RefinanceRateTerm,

    #[prompt("Refinance - Cash Out")]
    RefinanceCashOut {
        #[prompt("Cash Out Amount")]
        #[min(0)]
        cash_out_amount: i64,

        #[prompt("Purpose of Cash Out")]
        purpose: String,
    },

    #[prompt("Construction")]
    Construction,
}

#[derive(Wizard, Debug)]
enum LoanTerm {
    #[prompt("15 years")]
    Fifteen,

    #[prompt("20 years")]
    Twenty,

    #[prompt("30 years")]
    Thirty,
}

#[derive(Wizard, Debug)]
enum DownPaymentSource {
    #[prompt("Savings/Checking Account")]
    Savings,

    #[prompt("Sale of Current Home")]
    HomeSale,

    #[prompt("Gift from Family")]
    Gift {
        #[prompt("Donor Name")]
        donor_name: String,

        #[prompt("Relationship to Borrower")]
        relationship: String,

        #[prompt("Gift Amount")]
        #[min(0)]
        amount: i64,
    },

    #[prompt("Retirement Account")]
    Retirement,

    #[prompt("Foreign Bank Account")]
    ForeignAccount {
        #[prompt("Bank Name")]
        bank_name: String,

        #[prompt("Country")]
        country: String,
    },
}

#[derive(Wizard, Debug)]
struct FinancialInformation {
    #[prompt("Gross Monthly Income")]
    #[min(0)]
    gross_monthly_income: i64,

    #[prompt("Monthly Housing Expense (current)")]
    #[min(0)]
    current_housing_expense: i64,

    #[prompt("Do you pay alimony or child support?")]
    has_obligations: bool,

    #[prompt("Monthly Obligation Amount")]
    monthly_obligations: Option<i64>,
}

#[derive(Wizard, Debug)]
struct EmploymentHistory {
    #[prompt("Current Employment Status")]
    current_employment: EmploymentStatus,

    #[prompt("Years at Current Job")]
    #[min(0)]
    years_current: i64,

    #[prompt("Months at Current Job")]
    #[min(0)]
    #[max(11)]
    months_current: i64,
}

#[derive(Wizard, Debug)]
enum EmploymentStatus {
    #[prompt("Employed by Company")]
    Employed {
        #[prompt("Employer Name")]
        employer: String,

        #[prompt("Employer Address")]
        address: String,

        #[prompt("Position/Title")]
        position: String,

        #[prompt("Start Date")]
        start_date: String,

        #[prompt("Gross Monthly Income")]
        #[min(0)]
        monthly_income: i64,

        #[prompt("Employment Type")]
        employment_type: EmploymentType,

        #[prompt("Work Phone")]
        work_phone: String,
    },

    #[prompt("Self-Employed")]
    SelfEmployed {
        #[prompt("Business Name")]
        business_name: String,

        #[prompt("Type of Business")]
        business_type: String,

        #[prompt("Start Date")]
        start_date: String,

        #[prompt("Ownership Percentage")]
        #[min(0)]
        #[max(100)]
        ownership_pct: i64,

        #[prompt("Gross Monthly Income")]
        #[min(0)]
        monthly_income: i64,
    },

    #[prompt("Retired")]
    Retired {
        #[prompt("Monthly Retirement Income")]
        #[min(0)]
        monthly_income: i64,

        #[prompt("Income Source Details")]
        income_sources: RetirementIncome,
    },

    #[prompt("Not Employed")]
    NotEmployed {
        #[prompt("Reason for Unemployment")]
        reason: String,

        #[prompt("Alternative Income Source")]
        alt_income: String,
    },
}

#[derive(Wizard, Debug)]
enum EmploymentType {
    #[prompt("Full-Time Permanent")]
    FullTime,

    #[prompt("Part-Time")]
    PartTime,

    #[prompt("Contract/Temporary")]
    Contract,

    #[prompt("Seasonal")]
    Seasonal,
}

#[derive(Wizard, Debug)]
struct RetirementIncome {
    #[prompt("Social Security")]
    #[min(0)]
    social_security: i64,

    #[prompt("Pension")]
    #[min(0)]
    pension: i64,

    #[prompt("401(k)/IRA Distributions")]
    #[min(0)]
    retirement_distributions: i64,

    #[prompt("Other")]
    #[min(0)]
    other: i64,
}

#[derive(Wizard, Debug)]
struct Assets {
    #[prompt("Checking Accounts - Total Balance")]
    #[min(0)]
    checking: i64,

    #[prompt("Savings Accounts - Total Balance")]
    #[min(0)]
    savings: i64,

    #[prompt("Stocks/Bonds - Total Value")]
    #[min(0)]
    stocks_bonds: i64,

    #[prompt("Retirement Accounts (401k, IRA) - Total Value")]
    #[min(0)]
    retirement: i64,

    #[prompt("Do you own other real estate?")]
    has_other_property: bool,

    #[prompt("Other Real Estate Value")]
    other_real_estate: Option<i64>,

    #[prompt("Automobiles - Total Value")]
    #[min(0)]
    automobiles: i64,

    #[prompt("Other Assets")]
    #[min(0)]
    other_assets: i64,
}

#[derive(Wizard, Debug)]
struct Liabilities {
    #[prompt("Credit Cards - Total Monthly Payment")]
    #[min(0)]
    credit_cards: i64,

    #[prompt("Auto Loans - Total Monthly Payment")]
    #[min(0)]
    auto_loans: i64,

    #[prompt("Student Loans - Total Monthly Payment")]
    #[min(0)]
    student_loans: i64,

    #[prompt("Other Mortgages - Total Monthly Payment")]
    #[min(0)]
    other_mortgages: i64,

    #[prompt("Personal Loans - Total Monthly Payment")]
    #[min(0)]
    personal_loans: i64,

    #[prompt("Alimony/Child Support - Monthly Payment")]
    #[min(0)]
    support_payments: i64,
}

#[derive(Wizard, Debug)]
struct Declarations {
    #[prompt("Have you had any ownership interest in a property in the last 3 years?")]
    prior_property_ownership: bool,

    #[prompt("Are you a co-signer or guarantor on any debt?")]
    is_cosigner: bool,

    #[prompt("Are there any outstanding judgments against you?")]
    has_judgments: bool,

    #[prompt("Have you declared bankruptcy in the past 7 years?")]
    bankruptcy: bool,

    #[prompt("Have you had property foreclosed upon in the last 7 years?")]
    foreclosure: bool,

    #[prompt("Are you a party to a lawsuit?")]
    lawsuit: bool,

    #[prompt("Have you been delinquent on any federal debt?")]
    federal_debt_delinquent: bool,

    #[prompt("Are you obligated to pay child support or alimony?")]
    support_obligation: bool,

    #[prompt("Will you occupy the property as your primary residence?")]
    primary_residence_intent: bool,

    #[prompt("Have you had a short sale in the last 7 years?")]
    short_sale: bool,
}

fn main() {
    println!("=== COMPREHENSIVE MORTGAGE LOAN APPLICATION ===\n");
    println!("This example demonstrates an extremely complex real-world form.");
    println!("Based on mortgage applications for foreign nationals and visa holders.\n");

    #[cfg(feature = "typst-form")]
    {
        let typst_markup =
            MortgageLoanApplication::to_typst_form(Some("MORTGAGE LOAN APPLICATION"));
        std::fs::write("mortgage_application.typ", &typst_markup)
            .expect("Failed to write mortgage_application.typ");
        println!("✓ Generated Typst form: mortgage_application.typ");
        println!("  Compile with: typst compile mortgage_application.typ\n");
        println!("Form includes:");
        println!("  - Borrower Information (with citizenship/visa status)");
        println!("  - Property Details");
        println!("  - Loan Details");
        println!("  - Financial Information");
        println!("  - Employment History");
        println!("  - Assets and Liabilities");
        println!("  - Legal Declarations");
        println!("\n  Total estimated fields: 100+");
        println!("  Estimated completion time: 45-60 minutes");
    }

    #[cfg(not(feature = "typst-form"))]
    {
        println!("Running interactive interview...\n");
        println!("Note: This is a comprehensive form and will take significant time to complete.");
        println!("Press Ctrl+C to cancel at any time.\n");

        use derive_wizard::InterviewBuilder;

        let result = InterviewBuilder::new()
            .interview::<MortgageLoanApplication>()
            .build()
            .execute();

        match result {
            Ok(application) => {
                println!("\n=== MORTGAGE APPLICATION SUBMITTED ===");
                println!("{:#?}", application);
                println!("\nApplication will be processed within 43-47 days.");
                println!("You will be contacted for additional documentation if needed.");
            }
            Err(e) => {
                eprintln!("Error processing application: {}", e);
            }
        }
    }
}
