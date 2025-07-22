use emeraldc_lexer::{Span, WideTokenKind};
use emeraldc_parser::{
    Binary, BinaryOperator, Declaration, Expression, FatalParserError,
    Function, Identifier, Let, NodeError, Parenthesized, ParsedNode, Statement,
};
use maud::{PreEscaped, Render};

pub struct Visualizer<'s> {
    source: &'s str,
}

impl<'s> Visualizer<'s> {
    pub fn visualize(
        tree: impl Iterator<
            Item = Result<ParsedNode<Declaration>, FatalParserError>,
        >,
        source: &'s str,
    ) -> String {
        let mut visualizer = Self::new(source);
        let tree = visualizer.visualize_tree(tree);
        let tree_window = visualizer.visualize_tree_window(tree);
        let page = visualizer.visualize_page(tree_window);
        page.into_string()
    }

    fn new(source: &'s str) -> Self {
        Self { source }
    }

    fn visualize_tree(
        &mut self,
        tree: impl Iterator<
            Item = Result<ParsedNode<Declaration>, FatalParserError>,
        >,
    ) -> maud::Markup {
        maud::html! {
            @for declaration in tree {
                (declaration.visualize(self))
            }
        }
    }

    fn visualize_tree_window(
        &mut self,
        tree: maud::PreEscaped<String>,
    ) -> maud::Markup {
        maud::html! {
            div id="tree-window" {
                (tree)
            }
        }
    }

    fn visualize_page(
        &mut self,
        tree_window: impl maud::Render,
    ) -> maud::Markup {
        maud::html! {
            (maud::DOCTYPE)
            html {
                head {
                    meta charset="utf-8";
                    meta name="author" content="Georgiy Ozhegov";
                    title { "Parse Tree Visualization" }
                    style { (self.style()) }
                }
                body {
                    main {
                        (tree_window)
                        script { (self.script()) }
                    }
                }
            }
        }
    }

    fn style(&self) -> &str {
        include_str!("../style.css")
    }

    fn script(&self) -> &str {
        include_str!("../script.js")
    }

    pub fn text_of(&self, span: &Span) -> &'s str {
        &self.source[span.start..span.end]
    }
}

trait Visualize {
    fn visualize(self, v: &Visualizer) -> maud::Markup;
}

impl Visualize for Result<ParsedNode<Declaration>, FatalParserError> {
    fn visualize(self, v: &Visualizer) -> maud::Markup {
        let inner = match self {
            Ok(node) => node.visualize(v),
            Err(error) => error.visualize(v),
        };
        maud::html! {
            div class="pt-node" {
                (inner)
            }
        }
    }
}

impl<V> Visualize for ParsedNode<V>
where
    V: Visualize,
{
    fn visualize(self, v: &Visualizer) -> maud::Markup {
        let inner = match self.node {
            Ok(node) => node.visualize(v),
            Err(error) => error.visualize(v),
        };
        maud::html! {
            div class="pt-parsed-node" data-span-start=(self.span.start) data-span-end=(self.span.end) data-text=(v.text_of(&self.span)) {
                (inner)
            }
        }
    }
}

impl Visualize for Declaration {
    fn visualize(self, v: &Visualizer) -> maud::Markup {
        let inner = match self {
            Self::Function(function) => function.visualize(v),
        };
        maud::html! {
            div class="pt-declaration" {
                (inner)
            }
        }
    }
}

impl Visualize for Function {
    fn visualize(self, v: &Visualizer) -> maud::Markup {
        maud::html! {
            div class="pt-function"{
                (self._introducer.visualize(v))
                (self.identifier.visualize(v))
                (self._open_round.visualize(v))
                (self._close_round.visualize(v))
                (self.body.visualize(v))
                (self._end.visualize(v))
            }
        }
    }
}

impl Visualize for Vec<ParsedNode<Statement>> {
    fn visualize(self, v: &Visualizer) -> maud::Markup {
        maud::html! {
            div class="pt-function-body" {
                @for statement in self {
                    (statement.visualize(v))
                }
            }
        }
    }
}

impl Visualize for Statement {
    fn visualize(self, v: &Visualizer) -> maud::Markup {
        let inner = match self {
            Self::Let(let_) => let_.visualize(v),
        };
        maud::html! {
            div class="pt-statement" {
                (inner)
            }
        }
    }
}

impl Visualize for Let {
    fn visualize(self, v: &Visualizer) -> maud::Markup {
        maud::html! {
            div class="pt-let" {
                (self._introducer.visualize(v))
                (self.identifier.visualize(v))
                (self._equal.visualize(v))
                (self.value.visualize(v))
            }
        }
    }
}

impl Visualize for Expression {
    fn visualize(self, v: &Visualizer) -> maud::Markup {
        let inner = match self {
            Self::Integer => {
                maud::html! {
                    div class="pt-integer" {}
                }
            }
            Self::Variable(identifier) => identifier.visualize(v),
            Self::Binary(binary) => binary.visualize(v),
            Self::Parenthesized(parenthesized) => parenthesized.visualize(v),
        };
        maud::html! {
            div class="pt-expression" {
                (inner)
            }
        }
    }
}

impl Visualize for Binary {
    fn visualize(self, v: &Visualizer) -> maud::Markup {
        maud::html! {
            div class="pt-binary" {
                (self.left.visualize(v))
                (self.operator.visualize(v))
                (self.right.visualize(v))
            }
        }
    }
}

impl Visualize for BinaryOperator {
    fn visualize(self, _v: &Visualizer) -> maud::Markup {
        maud::html! {
            div class="pt-binary-operator" data-operator=(format!("{self:?}")) {}
        }
    }
}

impl Visualize for Parenthesized {
    fn visualize(self, v: &Visualizer) -> maud::Markup {
        maud::html! {
            div class="pt-parenthesized" {
                (self._open_round.visualize(v))
                (self.inner.visualize(v))
                (self._close_round.visualize(v))
            }
        }
    }
}

impl Visualize for Identifier {
    fn visualize(self, _v: &Visualizer) -> maud::Markup {
        maud::html! {
            div class="pt-identifier" {}
        }
    }
}

impl Visualize for WideTokenKind {
    fn visualize(self, _v: &Visualizer) -> maud::Markup {
        maud::html! {
            div class="pt-token" data-kind=(format!("{self:?}")) {}
        }
    }
}

impl Visualize for NodeError {
    fn visualize(self, _v: &Visualizer) -> maud::Markup {
        maud::html! {
            div class="pt-node-error" data-message=(self) {}
        }
    }
}

impl Visualize for FatalParserError {
    fn visualize(self, _v: &Visualizer) -> maud::Markup {
        maud::html! {
            div class="pt-fatal-error" data-message=(self) {}
        }
    }
}
