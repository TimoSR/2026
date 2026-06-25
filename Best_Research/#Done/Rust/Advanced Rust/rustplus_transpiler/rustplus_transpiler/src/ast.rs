#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TopLevelItem {
    Raw(String),
    Interface(TypeDeclaration),
    AbstractClass(TypeDeclaration),
    Class(TypeDeclaration),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeDeclaration {
    pub attributes: Vec<String>,
    pub visibility: Option<String>,
    pub name: String,
    pub generics_definition: String,
    pub generics_usage: String,
    pub bases: Vec<String>,
    pub body: String,
}

impl TypeDeclaration {
    pub fn has_generics(&self) -> bool {
        return !self.generics_definition.trim().is_empty();
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClassBody {
    pub fields: Vec<FieldDeclaration>,
    pub methods: Vec<MethodDeclaration>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldDeclaration {
    pub attributes: Vec<String>,
    pub source: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MethodDeclaration {
    pub attributes: Vec<String>,
    pub name: String,
    pub source: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParameterDeclaration {
    pub name: String,
    pub type_name: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeclarationKind {
    Interface,
    AbstractClass,
    Class,
}
