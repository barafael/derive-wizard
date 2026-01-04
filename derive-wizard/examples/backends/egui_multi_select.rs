//! Egui Multi-Select Example 🎨
//!
//! Demonstrates multi-select functionality with the egui GUI backend.
//!
//! Run with: cargo run --example egui_multi_select --features egui-backend

use derive_wizard::Wizard;

/// Programming languages for the survey
#[derive(Debug, Clone, Copy, Wizard)]
pub enum Language {
    Rust,
    Python,
    JavaScript,
    TypeScript,
    Go,
    Java,
    CSharp,
    Cpp,
    Ruby,
    Swift,
}

impl std::fmt::Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Language::Rust => write!(f, "Rust 🦀"),
            Language::Python => write!(f, "Python 🐍"),
            Language::JavaScript => write!(f, "JavaScript"),
            Language::TypeScript => write!(f, "TypeScript"),
            Language::Go => write!(f, "Go"),
            Language::Java => write!(f, "Java ☕"),
            Language::CSharp => write!(f, "C#"),
            Language::Cpp => write!(f, "C++"),
            Language::Ruby => write!(f, "Ruby 💎"),
            Language::Swift => write!(f, "Swift"),
        }
    }
}

/// Development tools
#[derive(Debug, Clone, Copy, Wizard)]
pub enum Tool {
    Git,
    Docker,
    Kubernetes,
    CICD,
    Terraform,
    Ansible,
    VSCode,
    Vim,
    JetBrains,
}

impl std::fmt::Display for Tool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tool::Git => write!(f, "Git"),
            Tool::Docker => write!(f, "Docker 🐳"),
            Tool::Kubernetes => write!(f, "Kubernetes ☸️"),
            Tool::CICD => write!(f, "CI/CD Pipelines"),
            Tool::Terraform => write!(f, "Terraform"),
            Tool::Ansible => write!(f, "Ansible"),
            Tool::VSCode => write!(f, "VS Code"),
            Tool::Vim => write!(f, "Vim/Neovim"),
            Tool::JetBrains => write!(f, "JetBrains IDEs"),
        }
    }
}

/// Areas of interest
#[derive(Debug, Clone, Copy, Wizard)]
pub enum Interest {
    WebDev,
    Backend,
    Frontend,
    Mobile,
    GameDev,
    MachineLearning,
    DataScience,
    DevOps,
    Security,
    Embedded,
}

impl std::fmt::Display for Interest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Interest::WebDev => write!(f, "Web Development"),
            Interest::Backend => write!(f, "Backend Systems"),
            Interest::Frontend => write!(f, "Frontend/UI"),
            Interest::Mobile => write!(f, "Mobile Development"),
            Interest::GameDev => write!(f, "Game Development 🎮"),
            Interest::MachineLearning => write!(f, "Machine Learning 🤖"),
            Interest::DataScience => write!(f, "Data Science 📊"),
            Interest::DevOps => write!(f, "DevOps & Infrastructure"),
            Interest::Security => write!(f, "Security 🔒"),
            Interest::Embedded => write!(f, "Embedded Systems"),
        }
    }
}

/// Experience level
#[derive(Debug, Clone, Copy, Default, Wizard)]
pub enum ExperienceLevel {
    Student,
    #[default]
    Junior,
    Mid,
    Senior,
    Lead,
    Principal,
}

/// Developer profile survey with multi-select fields
#[derive(Debug, Wizard)]
#[prelude(
    "Welcome! Tell us about your developer profile.\nThis helps us understand our community better."
)]
pub struct DeveloperProfile {
    #[prompt("What's your name?")]
    name: String,

    #[prompt("Years of programming experience:")]
    years_experience: u32,

    #[prompt("How would you describe your experience level?")]
    level: ExperienceLevel,

    #[prompt("Which programming languages do you use regularly?")]
    languages: Vec<Language>,

    #[prompt("Which tools are part of your daily workflow?")]
    tools: Vec<Tool>,

    #[prompt("What areas of development interest you most?")]
    interests: Vec<Interest>,

    #[prompt("Do you contribute to open source projects?")]
    open_source: bool,

    #[prompt("Are you open to new job opportunities?")]
    open_to_work: bool,

    #[prompt("Anything else you'd like to share?")]
    notes: String,
}

fn main() {
    println!("=== Developer Profile Survey - egui Multi-Select Demo ===");

    let backend = derive_wizard::EguiBackend::new()
        .with_title("Developer Profile Survey")
        .with_window_size([500.0, 400.0]);

    let profile = DeveloperProfile::wizard_builder()
        .with_backend(backend)
        .build()
        .unwrap();

    println!("\n🎉 Developer Profile Summary\n");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("👤 Name: {}", profile.name);
    println!(
        "📅 Experience: {} years ({:?})",
        profile.years_experience, profile.level
    );

    println!("\n💻 Languages ({}):", profile.languages.len());
    if profile.languages.is_empty() {
        println!("   (none selected)");
    } else {
        for lang in &profile.languages {
            println!("   • {}", lang);
        }
    }

    println!("\n🛠️  Tools ({}):", profile.tools.len());
    if profile.tools.is_empty() {
        println!("   (none selected)");
    } else {
        for tool in &profile.tools {
            println!("   • {}", tool);
        }
    }

    println!("\n🎯 Interests ({}):", profile.interests.len());
    if profile.interests.is_empty() {
        println!("   (none selected)");
    } else {
        for interest in &profile.interests {
            println!("   • {}", interest);
        }
    }

    println!("\n📝 Additional Info:");
    println!(
        "   Open Source Contributor: {}",
        if profile.open_source { "Yes ✓" } else { "No" }
    );
    println!(
        "   Open to Opportunities: {}",
        if profile.open_to_work {
            "Yes ✓"
        } else {
            "No"
        }
    );

    if !profile.notes.is_empty() {
        println!("\n💬 Notes: {}", profile.notes);
    }

    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
}
