use super::{
    import::{Import, RegisterImports},
    types::{PathType, Primitive, Type, Vector},
};
use crate::{
    impl_eq_name,
    models::{types::RenderedType, ComponentInfo, RenderedArgField},
    utils::html::to_html,
};

#[derive(Debug)]
pub struct Function {
    docs: Option<String>,
    name: String,
    args: Vec<Arg>,
    return_ty: Option<Type>,
}

impl Function {
    pub fn new(
        docs: Option<String>,
        name: String,
        args: Vec<Arg>,
        return_ty: Option<Type>,
    ) -> Function {
        Function {
            docs,
            name,
            args,
            return_ty,
        }
    }

    pub fn rendered_args(&self, imports: &[Import]) -> Vec<RenderedArgField> {
        self.args()
            .iter()
            .map(|arg| {
                let ty = match arg.argument_type() {
                    FunctionType::Primitive(p) => RenderedType {
                        name: p.to_string(),
                        ..Default::default()
                    },
                    FunctionType::Vector(v) => RenderedType {
                        name: v.to_string(),
                        ..Default::default()
                    },
                    FunctionType::Path(path) => {
                        Type::Path(path.clone()).rendered_type(imports, false)
                    }
                    FunctionType::FunctionPointer(inner_ty) => {
                        inner_ty.rendered_type(imports, true)
                    }
                };

                RenderedArgField {
                    docs: arg.docs().map(to_html),
                    name: arg.name().to_string(),
                    ty,
                }
            })
            .collect()
    }

    /// Returns a [`ComponentInfo`] containing a summary of the function documentation,
    /// with the summary extracted from the rendered Markdown as HTML.
    pub fn info_rich_text(&self) -> ComponentInfo {
        let summary = self.docs.as_deref().map(to_html);

        ComponentInfo::new(self.name.clone(), summary)
    }

    /// Returns a [`ComponentInfo`] containing a summary of the function documentation,
    /// with the summary extracted from the rendered Markdown as plain text. The summary is truncated
    /// to `ComponentInfo::SUMMARY_MAX_LENGTH` characters if necessary.
    pub fn info_plain_text(&self) -> ComponentInfo {
        let summary = self.docs.as_deref().map(|docs| {
            let html = to_html(docs);
            let parsed = scraper::Html::parse_fragment(&html);

            let summary = parsed
                .root_element()
                .text()
                .collect::<Vec<_>>()
                .join(" ")
                .trim()
                .to_string();

            if summary.len() > ComponentInfo::SUMMARY_MAX_LENGTH {
                format!("{}...", &summary[..ComponentInfo::SUMMARY_MAX_LENGTH])
            } else {
                summary
            }
        });

        ComponentInfo::new(self.name.clone(), summary)
    }

    pub fn docs(&self) -> Option<&str> {
        self.docs.as_deref()
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn args(&self) -> &[Arg] {
        &self.args
    }

    pub fn return_type(&self) -> Option<&Type> {
        self.return_ty.as_ref()
    }
}

impl RegisterImports for Function {
    fn register_imports(&mut self, imports: &[Import]) {
        for arg in &mut self.args {
            arg.register_imports(imports)
        }

        if let Some(Type::Path(path_type)) = &mut self.return_ty {
            path_type.register_imports(imports);
        }
    }

    fn register_same_module_types(&mut self, type_names: &[String]) {
        for arg in &mut self.args {
            arg.register_same_module_types(type_names);
        }

        if let Some(Type::Path(path_type)) = &mut self.return_ty {
            path_type.register_same_module_types(type_names);
        }
    }
}

impl_eq_name!(Function::name);

#[derive(Debug)]
pub struct Arg {
    docs: Option<String>,
    name: String,
    ty: FunctionType,
}

impl Arg {
    pub fn new(docs: Option<String>, name: String, ty: FunctionType) -> Arg {
        Arg { docs, name, ty }
    }

    pub fn docs(&self) -> Option<&str> {
        self.docs.as_deref()
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn argument_type(&self) -> &FunctionType {
        &self.ty
    }
}

impl RegisterImports for Arg {
    fn register_imports(&mut self, imports: &[Import]) {
        match &mut self.ty {
            FunctionType::FunctionPointer(Type::Path(ref mut path_type)) => {
                path_type.register_imports(imports);
            }
            FunctionType::Path(ref mut ty) => ty.register_imports(imports),
            _ => {}
        }
    }

    fn register_same_module_types(&mut self, type_names: &[String]) {
        match &mut self.ty {
            FunctionType::FunctionPointer(Type::Path(ref mut path_type)) => {
                path_type.register_same_module_types(type_names);
            }
            FunctionType::Path(ref mut ty) => ty.register_same_module_types(type_names),
            _ => {}
        }
    }
}

impl_eq_name!(Arg::name);

#[derive(Debug)]
pub enum FunctionType {
    Primitive(Primitive),
    Vector(Vector),
    Path(PathType),
    FunctionPointer(Type),
}

impl Default for FunctionType {
    fn default() -> Self {
        FunctionType::Primitive(Primitive::default())
    }
}
