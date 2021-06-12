//! Re-export diagnostics such that clients of `hir` don't have to depend on
//! low-level crates.
//!
//! This probably isn't the best way to do this -- ideally, diagnistics should
//! be expressed in terms of hir types themselves.
use std::any::Any;

use cfg::{CfgExpr, CfgOptions, DnfExpr};
use hir_def::path::ModPath;
use hir_expand::{HirFileId, InFile};
use stdx::format_to;
use syntax::{ast, AstPtr, SyntaxNodePtr, TextRange};

pub use hir_ty::{
    diagnostics::{
        IncorrectCase, MismatchedArgCount, MissingFields, MissingMatchArms,
        MissingOkOrSomeInTailExpr, RemoveThisSemicolon, ReplaceFilterMapNextWithFindMap,
    },
    diagnostics_sink::{Diagnostic, DiagnosticCode, DiagnosticSink, DiagnosticSinkBuilder},
};

// Diagnostic: unresolved-module
//
// This diagnostic is triggered if rust-analyzer is unable to discover referred module.
#[derive(Debug)]
pub struct UnresolvedModule {
    pub file: HirFileId,
    pub decl: AstPtr<ast::Module>,
    pub candidate: String,
}

impl Diagnostic for UnresolvedModule {
    fn code(&self) -> DiagnosticCode {
        DiagnosticCode("unresolved-module")
    }
    fn message(&self) -> String {
        "unresolved module".to_string()
    }
    fn display_source(&self) -> InFile<SyntaxNodePtr> {
        InFile::new(self.file, self.decl.clone().into())
    }
    fn as_any(&self) -> &(dyn Any + Send + 'static) {
        self
    }
}

// Diagnostic: unresolved-extern-crate
//
// This diagnostic is triggered if rust-analyzer is unable to discover referred extern crate.
#[derive(Debug)]
pub struct UnresolvedExternCrate {
    pub file: HirFileId,
    pub item: AstPtr<ast::ExternCrate>,
}

impl Diagnostic for UnresolvedExternCrate {
    fn code(&self) -> DiagnosticCode {
        DiagnosticCode("unresolved-extern-crate")
    }
    fn message(&self) -> String {
        "unresolved extern crate".to_string()
    }
    fn display_source(&self) -> InFile<SyntaxNodePtr> {
        InFile::new(self.file, self.item.clone().into())
    }
    fn as_any(&self) -> &(dyn Any + Send + 'static) {
        self
    }
}

#[derive(Debug)]
pub struct UnresolvedImport {
    pub file: HirFileId,
    pub node: AstPtr<ast::UseTree>,
}

impl Diagnostic for UnresolvedImport {
    fn code(&self) -> DiagnosticCode {
        DiagnosticCode("unresolved-import")
    }
    fn message(&self) -> String {
        "unresolved import".to_string()
    }
    fn display_source(&self) -> InFile<SyntaxNodePtr> {
        InFile::new(self.file, self.node.clone().into())
    }
    fn as_any(&self) -> &(dyn Any + Send + 'static) {
        self
    }
    fn is_experimental(&self) -> bool {
        // This currently results in false positives in the following cases:
        // - `cfg_if!`-generated code in libstd (we don't load the sysroot correctly)
        // - `core::arch` (we don't handle `#[path = "../<path>"]` correctly)
        // - proc macros and/or proc macro generated code
        true
    }
}

// Diagnostic: unresolved-macro-call
//
// This diagnostic is triggered if rust-analyzer is unable to resolve the path to a
// macro in a macro invocation.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct UnresolvedMacroCall {
    pub file: HirFileId,
    pub node: AstPtr<ast::MacroCall>,
    pub path: ModPath,
}

impl Diagnostic for UnresolvedMacroCall {
    fn code(&self) -> DiagnosticCode {
        DiagnosticCode("unresolved-macro-call")
    }
    fn message(&self) -> String {
        format!("unresolved macro `{}!`", self.path)
    }
    fn display_source(&self) -> InFile<SyntaxNodePtr> {
        InFile::new(self.file, self.node.clone().into())
    }
    fn as_any(&self) -> &(dyn Any + Send + 'static) {
        self
    }
    fn is_experimental(&self) -> bool {
        true
    }
}

// Diagnostic: inactive-code
//
// This diagnostic is shown for code with inactive `#[cfg]` attributes.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct InactiveCode {
    pub file: HirFileId,
    pub node: SyntaxNodePtr,
    pub cfg: CfgExpr,
    pub opts: CfgOptions,
}

impl Diagnostic for InactiveCode {
    fn code(&self) -> DiagnosticCode {
        DiagnosticCode("inactive-code")
    }
    fn message(&self) -> String {
        let inactive = DnfExpr::new(self.cfg.clone()).why_inactive(&self.opts);
        let mut buf = "code is inactive due to #[cfg] directives".to_string();

        if let Some(inactive) = inactive {
            format_to!(buf, ": {}", inactive);
        }

        buf
    }
    fn display_source(&self) -> InFile<SyntaxNodePtr> {
        InFile::new(self.file, self.node.clone())
    }
    fn as_any(&self) -> &(dyn Any + Send + 'static) {
        self
    }
}

// Diagnostic: unresolved-proc-macro
//
// This diagnostic is shown when a procedural macro can not be found. This usually means that
// procedural macro support is simply disabled (and hence is only a weak hint instead of an error),
// but can also indicate project setup problems.
//
// If you are seeing a lot of "proc macro not expanded" warnings, you can add this option to the
// `rust-analyzer.diagnostics.disabled` list to prevent them from showing. Alternatively you can
// enable support for procedural macros (see `rust-analyzer.procMacro.enable`).
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct UnresolvedProcMacro {
    pub file: HirFileId,
    pub node: SyntaxNodePtr,
    /// If the diagnostic can be pinpointed more accurately than via `node`, this is the `TextRange`
    /// to use instead.
    pub precise_location: Option<TextRange>,
    pub macro_name: Option<String>,
}

impl Diagnostic for UnresolvedProcMacro {
    fn code(&self) -> DiagnosticCode {
        DiagnosticCode("unresolved-proc-macro")
    }

    fn message(&self) -> String {
        match &self.macro_name {
            Some(name) => format!("proc macro `{}` not expanded", name),
            None => "proc macro not expanded".to_string(),
        }
    }

    fn display_source(&self) -> InFile<SyntaxNodePtr> {
        InFile::new(self.file, self.node.clone())
    }

    fn as_any(&self) -> &(dyn Any + Send + 'static) {
        self
    }
}

// Diagnostic: macro-error
//
// This diagnostic is shown for macro expansion errors.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MacroError {
    pub file: HirFileId,
    pub node: SyntaxNodePtr,
    pub message: String,
}

impl Diagnostic for MacroError {
    fn code(&self) -> DiagnosticCode {
        DiagnosticCode("macro-error")
    }
    fn message(&self) -> String {
        self.message.clone()
    }
    fn display_source(&self) -> InFile<SyntaxNodePtr> {
        InFile::new(self.file, self.node.clone())
    }
    fn as_any(&self) -> &(dyn Any + Send + 'static) {
        self
    }
    fn is_experimental(&self) -> bool {
        // Newly added and not very well-tested, might contain false positives.
        true
    }
}

#[derive(Debug)]
pub struct UnimplementedBuiltinMacro {
    pub file: HirFileId,
    pub node: SyntaxNodePtr,
}

impl Diagnostic for UnimplementedBuiltinMacro {
    fn code(&self) -> DiagnosticCode {
        DiagnosticCode("unimplemented-builtin-macro")
    }

    fn message(&self) -> String {
        "unimplemented built-in macro".to_string()
    }

    fn display_source(&self) -> InFile<SyntaxNodePtr> {
        InFile::new(self.file, self.node.clone())
    }

    fn as_any(&self) -> &(dyn Any + Send + 'static) {
        self
    }
}

// Diagnostic: no-such-field
//
// This diagnostic is triggered if created structure does not have field provided in record.
#[derive(Debug)]
pub struct NoSuchField {
    pub file: HirFileId,
    pub field: AstPtr<ast::RecordExprField>,
}

impl Diagnostic for NoSuchField {
    fn code(&self) -> DiagnosticCode {
        DiagnosticCode("no-such-field")
    }

    fn message(&self) -> String {
        "no such field".to_string()
    }

    fn display_source(&self) -> InFile<SyntaxNodePtr> {
        InFile::new(self.file, self.field.clone().into())
    }

    fn as_any(&self) -> &(dyn Any + Send + 'static) {
        self
    }
}

// Diagnostic: break-outside-of-loop
//
// This diagnostic is triggered if the `break` keyword is used outside of a loop.
#[derive(Debug)]
pub struct BreakOutsideOfLoop {
    pub file: HirFileId,
    pub expr: AstPtr<ast::Expr>,
}

impl Diagnostic for BreakOutsideOfLoop {
    fn code(&self) -> DiagnosticCode {
        DiagnosticCode("break-outside-of-loop")
    }
    fn message(&self) -> String {
        "break outside of loop".to_string()
    }
    fn display_source(&self) -> InFile<SyntaxNodePtr> {
        InFile { file_id: self.file, value: self.expr.clone().into() }
    }
    fn as_any(&self) -> &(dyn Any + Send + 'static) {
        self
    }
}

// Diagnostic: missing-unsafe
//
// This diagnostic is triggered if an operation marked as `unsafe` is used outside of an `unsafe` function or block.
#[derive(Debug)]
pub struct MissingUnsafe {
    pub file: HirFileId,
    pub expr: AstPtr<ast::Expr>,
}

impl Diagnostic for MissingUnsafe {
    fn code(&self) -> DiagnosticCode {
        DiagnosticCode("missing-unsafe")
    }
    fn message(&self) -> String {
        format!("This operation is unsafe and requires an unsafe function or block")
    }
    fn display_source(&self) -> InFile<SyntaxNodePtr> {
        InFile { file_id: self.file, value: self.expr.clone().into() }
    }
    fn as_any(&self) -> &(dyn Any + Send + 'static) {
        self
    }
}
