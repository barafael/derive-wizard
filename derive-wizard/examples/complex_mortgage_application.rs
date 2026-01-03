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
    #[prompt("PRIMARY BORROWER INFORMATION")]
    primary_borrower: BorrowerInfo,

    #[prompt("Do you have a co-borrower?")]
    has_co_borrower: bool,

    #[prompt("PROPERTY INFORMATION")]
    property: PropertyInfo,

    #[prompt("LOAN DETAILS")]
    loan_details: LoanDetails,

    #[prompt("EMPLOYMENT AND INCOME")]
    employment: EmploymentInfo,

    #[prompt("FINANCIAL ASSETS")]
    assets: AssetInfo,

    #[prompt("LIABILITIES")]
    liabilities: LiabilityInfo,

    #[prompt("DECLARATIONS")]
    declarations: DeclarationInfo,
}

#[derive(Wizard, Debug)]
struct BorrowerInfo {
    #[prompt("Full Legal Name (as appears on passport)")]
    full_name: String,

    #[prompt("Date of Birth (MM/DD/YYYY)")]
    date_of_birth: String,

    #[prompt("Social Security Number or ITIN")]
    ssn_or_itin: String,

    #[prompt("Citizenship Status")]
    citizenship: CitizenshipStatus,

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

    #[prompt("Permanent Resident (Green Card)")]
    PermanentResident {
        #[prompt("Green Card Number")]
        green_card_number: String,

        #[prompt("Expiration Date (MM/DD/YYYY)")]
        expiration_date: String,
    },

    #[prompt("Visa Holder")]
    VisaHolder {
        #[prompt("Visa Type")]
        visa_type: VisaType,

        #[prompt("Passport Number")]
        passport_number: String,

        #[prompt("Passport Country")]
        passport_country: String,

        #[prompt("Visa Expiration Date (MM/DD/YYYY)")]
        visa_expiration: String,

        #[prompt("Years remaining on visa")]
        #[min(0)]
        years_remaining: i64,
    },

    #[prompt("Foreign National")]
    ForeignNational {
        #[prompt("Country of Residence")]
        country: String,

        #[prompt("Passport Number")]
        passport_number: String,
    },
}

#[derive(Wizard, Debug)]
enum VisaType {
    #[prompt("H-1B (Specialty Occupation)")]
    H1B,

    #[prompt("L-1 (Intracompany Transfer)")]
    L1,

    #[prompt("E-2 (Treaty Investor)")]
    E2,

    #[prompt("O-1 (Extraordinary Ability)")]
    O1,

    #[prompt("F-1 (Student with OPT)")]
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
    #[prompt("Single Family Home")]
    SingleFamily,

    Condominium,
    Townhouse,

    #[prompt("Multi-Family (2-4 units)")]
    MultiFamily,
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

    #[prompt("Loan Term")]
    loan_term: LoanTerm,

    #[prompt("Down Payment Amount")]
    #[min(0)]
    down_payment: i64,

    #[prompt("Down Payment Source")]
    down_payment_source: DownPaymentSource,
}

#[derive(Wizard, Debug)]
enum LoanPurpose {
    Purchase,

    #[prompt("Refinance")]
    Refinance {
        #[prompt("Current Loan Balance")]
        #[min(0)]
        current_balance: i64,

        #[prompt("Cash out amount")]
        #[min(0)]
        cash_out: i64,
    },

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
    #[prompt("Savings Account")]
    Savings,

    #[prompt("Sale of Current Home")]
    HomeSale,

    #[prompt("Gift from Family")]
    Gift {
        #[prompt("Donor Name")]
        donor_name: String,

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
struct EmploymentInfo {
    #[prompt("Current Employment Status")]
    current_status: EmploymentStatus,

    #[prompt("Gross Monthly Income")]
    #[min(0)]
    monthly_income: i64,

    #[prompt("Years at Current Job")]
    #[min(0)]
    years_employed: i64,

    #[prompt("Months at Current Job")]
    #[min(0)]
    #[max(11)]
    months_employed: i64,
}

#[derive(Wizard, Debug)]
enum EmploymentStatus {
    #[prompt("Employed Full-Time")]
    Employed {
        #[prompt("Employer Name")]
        employer: String,

        #[prompt("Job Title")]
        position: String,

        #[prompt("Work Phone")]
        work_phone: String,
    },

    #[prompt("Self-Employed")]
    SelfEmployed {
        #[prompt("Business Name")]
        business_name: String,

        #[prompt("Type of Business")]
        business_type: String,

        #[prompt("Years in Business")]
        #[min(0)]
        years_in_business: i64,
    },

    Retired {
        #[prompt("Monthly Pension Amount")]
        #[min(0)]
        pension: i64,

        #[prompt("Monthly Social Security")]
        #[min(0)]
        social_security: i64,
    },

    #[prompt("Not Currently Employed")]
    Unemployed,
}

#[derive(Wizard, Debug)]
struct AssetInfo {
    #[prompt("Checking Accounts Total")]
    #[min(0)]
    checking: i64,

    #[prompt("Savings Accounts Total")]
    #[min(0)]
    savings: i64,

    #[prompt("Stocks and Bonds Total")]
    #[min(0)]
    stocks_bonds: i64,

    #[prompt("Retirement Accounts Total (401k, IRA)")]
    #[min(0)]
    retirement: i64,

    #[prompt("Real Estate Owned Value")]
    #[min(0)]
    real_estate: i64,

    #[prompt("Automobiles Total Value")]
    #[min(0)]
    automobiles: i64,

    #[prompt("Other Assets Total")]
    #[min(0)]
    other_assets: i64,
}

#[derive(Wizard, Debug)]
struct LiabilityInfo {
    #[prompt("Credit Cards - Monthly Payment")]
    #[min(0)]
    credit_cards: i64,

    #[prompt("Auto Loans - Monthly Payment")]
    #[min(0)]
    auto_loans: i64,

    #[prompt("Student Loans - Monthly Payment")]
    #[min(0)]
    student_loans: i64,

    #[prompt("Other Mortgages - Monthly Payment")]
    #[min(0)]
    other_mortgages: i64,

    #[prompt("Personal Loans - Monthly Payment")]
    #[min(0)]
    personal_loans: i64,

    #[prompt("Alimony or Child Support - Monthly Payment")]
    #[min(0)]
    support_payments: i64,
}

#[derive(Wizard, Debug)]
struct DeclarationInfo {
    #[prompt("Have you had ownership in property in last 3 years?")]
    prior_property_ownership: bool,

    #[prompt("Are you a co-signer on any debt?")]
    is_cosigner: bool,

    #[prompt("Are there outstanding judgments against you?")]
    has_judgments: bool,

    #[prompt("Declared bankruptcy in past 7 years?")]
    bankruptcy: bool,

    #[prompt("Property foreclosed in last 7 years?")]
    foreclosure: bool,

    #[prompt("Currently party to a lawsuit?")]
    lawsuit: bool,

    #[prompt("Delinquent on federal debt?")]
    federal_debt_delinquent: bool,

    #[prompt("Obligated to pay alimony or child support?")]
    support_obligation: bool,

    #[prompt("Will this be your primary residence?")]
    primary_residence_intent: bool,
}

fn main() {
    println!("=== COMPREHENSIVE MORTGAGE LOAN APPLICATION ===\n");
    println!("This demonstrates a complex real-world form with 80+ fields.");
    println!("Based on mortgage applications for foreign nationals and visa holders.\n");

    #[cfg(feature = "typst-form")]
    {
        let typst_markup =
            MortgageLoanApplication::to_typst_form(Some("MORTGAGE LOAN APPLICATION"));
        std::fs::write("mortgage_application.typ", &typst_markup)
            .expect("Failed to write mortgage_application.typ");
        println!("✓ Generated Typst form: mortgage_application.typ");
        println!("  Compile with: typst compile mortgage_application.typ\n");
        println!("Form sections:");
        println!("  - Borrower Information (citizenship/visa status)");
        println!("  - Property Details");
        println!("  - Loan Details & Purpose");
        println!("  - Employment & Income");
        println!("  - Assets (7 categories)");
        println!("  - Liabilities (6 categories)");
        println!("  - Legal Declarations (9 questions)");
        println!("\n  Total fields: 80+");
        println!("  Estimated completion time: 30-45 minutes");
    }

    #[cfg(not(feature = "typst-form"))]
    {
        println!("Running interactive interview...\n");

        use derive_wizard::InterviewBuilder;

        let result = InterviewBuilder::new()
            .interview::<MortgageLoanApplication>()
            .build()
            .execute();

        match result {
            Ok(application) => {
                println!("\n=== MORTGAGE APPLICATION SUBMITTED ===");
                println!("{:#?}", application);
                println!("\nProcessing time: 43-47 days.");
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
}
