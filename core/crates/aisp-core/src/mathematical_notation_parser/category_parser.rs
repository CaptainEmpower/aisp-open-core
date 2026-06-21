//! Category Theory Expression Parser
//!
//! Specialized parser for category theory mathematical notation and constructs.

use super::types::*;
use crate::error::AispResult;
use std::iter::Peekable;
use std::str::Chars;

/// Category theory expression parser
pub struct CategoryTheoryParser {
    /// Category theory symbol registry
    category_registry: std::collections::HashMap<String, CategorySymbolInfo>,
}

impl CategoryTheoryParser {
    /// Create new category theory parser
    pub fn new() -> Self {
        Self {
            category_registry: Self::create_category_registry(),
        }
    }

    /// Parse category theory construct
    pub fn parse_category_theory_construct(
        &self,
        chars: &mut Peekable<Chars>,
        context: &mut ParsingContext,
    ) -> AispResult<EnhancedMathExpression> {
        let start_char = chars.peek().cloned().unwrap();

        match start_char {
            '𝔽' | '𝔾' => self.parse_functor(chars, context),
            '⟨' => self.parse_categorical_tuple(chars, context),
            '⇒' => self.parse_natural_transformation(chars, context),
            '⊣' => self.parse_adjunction(chars, context),
            '∘' => self.parse_composition(chars, context),
            _ => self.parse_basic_category_symbol(chars, context),
        }
    }

    /// Parse functor notation: 𝔽:𝐁𝐥𝐤⇒𝐕𝐚𝐥
    pub fn parse_functor(
        &self,
        chars: &mut Peekable<Chars>,
        context: &mut ParsingContext,
    ) -> AispResult<EnhancedMathExpression> {
        let functor_name = chars.next().unwrap().to_string();
        context.position += 1;
        self.skip_whitespace(chars, context);

        if chars.peek() == Some(&':') {
            chars.next(); // consume ':'
            context.position += 1;
            self.skip_whitespace(chars, context);

            let source = self.parse_category_name(chars, context)?;
            self.skip_whitespace(chars, context);

            if chars.peek() == Some(&'⇒') {
                chars.next(); // consume '⇒'
                context.position += 1;
                self.skip_whitespace(chars, context);

                let target = self.parse_category_name(chars, context)?;

                Ok(EnhancedMathExpression::CategoryTheory {
                    construct: CategoryConstruct::Functor {
                        name: functor_name,
                        source,
                        target,
                    },
                })
            } else {
                context.add_error(MathNotationError::InvalidExpression {
                    expression: "functor definition".to_string(),
                    reason: "Expected ⇒ after source category".to_string(),
                });
                Ok(EnhancedMathExpression::BasicSymbol(functor_name))
            }
        } else {
            // Just a functor symbol without explicit typing
            Ok(EnhancedMathExpression::BasicSymbol(functor_name))
        }
    }

    /// Parse categorical tuple: ⟨Objects, Morphisms, ∘, id⟩
    pub fn parse_categorical_tuple(
        &self,
        chars: &mut Peekable<Chars>,
        context: &mut ParsingContext,
    ) -> AispResult<EnhancedMathExpression> {
        chars.next(); // consume '⟨'
        context.position += 1;
        let mut components = Vec::new();

        loop {
            self.skip_whitespace(chars, context);

            if chars.peek() == Some(&'⟩') {
                chars.next();
                context.position += 1;
                break;
            }

            let component = self.parse_category_component(chars, context)?;
            components.push(component);

            self.skip_whitespace(chars, context);

            if chars.peek() == Some(&',') {
                chars.next();
                context.position += 1;
            } else if chars.peek() != Some(&'⟩') {
                context.add_error(MathNotationError::InvalidExpression {
                    expression: "categorical tuple".to_string(),
                    reason: "Expected ',' or '⟩'".to_string(),
                });
                break;
            }
        }

        // Try to interpret as specific category structures
        if components.len() == 4 {
            Ok(EnhancedMathExpression::CategoryTheory {
                construct: CategoryConstruct::Category {
                    name: "Category".to_string(),
                    objects: self.component_to_string(&components[0]),
                    morphisms: self.component_to_string(&components[1]),
                    composition: self.component_to_string(&components[2]),
                    identity: self.component_to_string(&components[3]),
                },
            })
        } else if components.len() == 3 {
            Ok(EnhancedMathExpression::CategoryTheory {
                construct: CategoryConstruct::Monad {
                    endofunctor: self.component_to_string(&components[0]),
                    unit: self.component_to_string(&components[1]),
                    multiplication: self.component_to_string(&components[2]),
                },
            })
        } else {
            Ok(EnhancedMathExpression::ComplexStructure {
                structure_type: "categorical_tuple".to_string(),
                components,
            })
        }
    }

    /// Parse natural transformation: η: F ⇒ G
    pub fn parse_natural_transformation(
        &self,
        chars: &mut Peekable<Chars>,
        context: &mut ParsingContext,
    ) -> AispResult<EnhancedMathExpression> {
        chars.next(); // consume '⇒'
        context.position += 1;

        // This is part of a larger expression, we return the symbol
        Ok(EnhancedMathExpression::UnicodeOperator {
            symbol: "⇒".to_string(),
            unicode_name: "RIGHTWARDS DOUBLE ARROW".to_string(),
            category: "Natural Transformation".to_string(),
        })
    }

    /// Parse adjunction: L ⊣ R
    pub fn parse_adjunction(
        &self,
        chars: &mut Peekable<Chars>,
        context: &mut ParsingContext,
    ) -> AispResult<EnhancedMathExpression> {
        chars.next(); // consume '⊣'
        context.position += 1;

        Ok(EnhancedMathExpression::UnicodeOperator {
            symbol: "⊣".to_string(),
            unicode_name: "LEFT TACK".to_string(),
            category: "Adjunction".to_string(),
        })
    }

    /// Parse composition: g ∘ f
    pub fn parse_composition(
        &self,
        chars: &mut Peekable<Chars>,
        context: &mut ParsingContext,
    ) -> AispResult<EnhancedMathExpression> {
        chars.next(); // consume '∘'
        context.position += 1;

        Ok(EnhancedMathExpression::UnicodeOperator {
            symbol: "∘".to_string(),
            unicode_name: "RING OPERATOR".to_string(),
            category: "Composition".to_string(),
        })
    }

    /// Parse basic category symbol
    fn parse_basic_category_symbol(
        &self,
        chars: &mut Peekable<Chars>,
        context: &mut ParsingContext,
    ) -> AispResult<EnhancedMathExpression> {
        let symbol = chars.next().unwrap().to_string();
        context.position += 1;

        if let Some(category_info) = self.category_registry.get(&symbol) {
            Ok(EnhancedMathExpression::CategoryTheory {
                construct: self.create_construct_from_symbol(&symbol, category_info),
            })
        } else {
            Ok(EnhancedMathExpression::BasicSymbol(symbol))
        }
    }

    /// Parse category name
    fn parse_category_name(
        &self,
        chars: &mut Peekable<Chars>,
        context: &mut ParsingContext,
    ) -> AispResult<String> {
        let mut name = String::new();

        while let Some(&ch) = chars.peek() {
            if ch.is_alphanumeric() || ch == '_' || self.is_mathematical_letter(ch) {
                name.push(chars.next().unwrap());
                context.position += 1;
            } else {
                break;
            }
        }

        if name.is_empty() {
            context.add_error(MathNotationError::InvalidExpression {
                expression: "category name".to_string(),
                reason: "Empty category name".to_string(),
            });
            Ok("Unknown".to_string())
        } else {
            Ok(name)
        }
    }

    /// Parse category component (within tuple)
    fn parse_category_component(
        &self,
        chars: &mut Peekable<Chars>,
        context: &mut ParsingContext,
    ) -> AispResult<EnhancedMathExpression> {
        let mut component = String::new();

        while let Some(&ch) = chars.peek() {
            if ch == ',' || ch == '⟩' {
                break;
            }
            component.push(chars.next().unwrap());
            context.position += 1;
        }

        Ok(EnhancedMathExpression::BasicSymbol(
            component.trim().to_string(),
        ))
    }

    /// Convert component to string representation
    fn component_to_string(&self, component: &EnhancedMathExpression) -> String {
        match component {
            EnhancedMathExpression::BasicSymbol(s) => s.clone(),
            EnhancedMathExpression::UnicodeOperator { symbol, .. } => symbol.clone(),
            _ => "component".to_string(),
        }
    }

    /// Create construct from symbol and category info
    fn create_construct_from_symbol(
        &self,
        symbol: &str,
        _category_info: &CategorySymbolInfo,
    ) -> CategoryConstruct {
        match symbol {
            "∘" => CategoryConstruct::Composition {
                functions: Vec::new(),
            },
            "id" => CategoryConstruct::Identity {
                object: "A".to_string(),
            },
            _ => CategoryConstruct::Morphism {
                name: symbol.to_string(),
                source: "A".to_string(),
                target: "B".to_string(),
            },
        }
    }

    /// Check if character is a mathematical letter
    fn is_mathematical_letter(&self, ch: char) -> bool {
        matches!(ch as u32,
            0x1D400..=0x1D7FF | // Mathematical Alphanumeric Symbols
            0x2102..=0x2138     // Letterlike Symbols
        )
    }

    /// Skip whitespace characters
    fn skip_whitespace(&self, chars: &mut Peekable<Chars>, context: &mut ParsingContext) {
        while let Some(&ch) = chars.peek() {
            if ch.is_whitespace() {
                chars.next();
                context.position += 1;
            } else {
                break;
            }
        }
    }

    /// Create category theory symbol registry
    fn create_category_registry() -> std::collections::HashMap<String, CategorySymbolInfo> {
        let mut registry = std::collections::HashMap::new();

        // Composition
        registry.insert(
            "∘".to_string(),
            CategorySymbolInfo::new(
                "∘".to_string(),
                "Morphism composition".to_string(),
                "Category theory".to_string(),
            )
            .with_definition(
                "Composition of morphisms f: A → B and g: B → C gives g ∘ f: A → C".to_string(),
            )
            .with_related_concept("Morphism".to_string())
            .with_related_concept("Category".to_string()),
        );

        // Natural transformation
        registry.insert(
            "⇒".to_string(),
            CategorySymbolInfo::new(
                "⇒".to_string(),
                "Natural transformation".to_string(),
                "Functor category".to_string(),
            )
            .with_definition(
                "Natural transformation η: F ⇒ G between functors F, G: C → D".to_string(),
            )
            .with_related_concept("Functor".to_string())
            .with_related_concept("Natural isomorphism".to_string()),
        );

        // Adjunction
        registry.insert(
            "⊣".to_string(),
            CategorySymbolInfo::new(
                "⊣".to_string(),
                "Adjunction".to_string(),
                "Adjoint functors".to_string(),
            )
            .with_definition("L ⊣ R means L is left adjoint to R".to_string())
            .with_related_concept("Left adjoint".to_string())
            .with_related_concept("Right adjoint".to_string()),
        );

        // Functor arrow
        registry.insert(
            "→".to_string(),
            CategorySymbolInfo::new(
                "→".to_string(),
                "Morphism or functor".to_string(),
                "Category theory".to_string(),
            )
            .with_definition("f: A → B represents a morphism from A to B".to_string())
            .with_related_concept("Morphism".to_string())
            .with_related_concept("Object".to_string()),
        );

        registry
    }
}

impl Default for CategoryTheoryParser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_context() -> ParsingContext {
        ParsingContext::new()
    }

    #[test]
    fn test_category_parser_creation() {
        let parser = CategoryTheoryParser::new();
        assert!(!parser.category_registry.is_empty());
    }

    #[test]
    fn test_composition_parsing() {
        let parser = CategoryTheoryParser::new();
        let mut context = create_test_context();
        let mut chars = "∘".chars().peekable();

        let result = parser.parse_composition(&mut chars, &mut context);
        assert!(result.is_ok());

        let expr = result.unwrap();
        assert!(matches!(
            expr,
            EnhancedMathExpression::UnicodeOperator { .. }
        ));
    }

    #[test]
    fn test_functor_parsing() {
        let parser = CategoryTheoryParser::new();
        let mut context = create_test_context();
        let mut chars = "𝔽:C⇒D".chars().peekable();

        let result = parser.parse_functor(&mut chars, &mut context);
        assert!(result.is_ok());

        let expr = result.unwrap();
        assert!(matches!(
            expr,
            EnhancedMathExpression::CategoryTheory { .. }
        ));
    }

    #[test]
    fn test_categorical_tuple_parsing() {
        let parser = CategoryTheoryParser::new();
        let mut context = create_test_context();
        let mut chars = "⟨T,η,μ⟩".chars().peekable();

        let result = parser.parse_categorical_tuple(&mut chars, &mut context);
        assert!(result.is_ok());

        let expr = result.unwrap();
        assert!(matches!(
            expr,
            EnhancedMathExpression::CategoryTheory { .. }
        ));
    }

    #[test]
    fn test_adjunction_parsing() {
        let parser = CategoryTheoryParser::new();
        let mut context = create_test_context();
        let mut chars = "⊣".chars().peekable();

        let result = parser.parse_adjunction(&mut chars, &mut context);
        assert!(result.is_ok());

        let expr = result.unwrap();
        assert!(matches!(
            expr,
            EnhancedMathExpression::UnicodeOperator { .. }
        ));
    }

    #[test]
    fn test_natural_transformation_parsing() {
        let parser = CategoryTheoryParser::new();
        let mut context = create_test_context();
        let mut chars = "⇒".chars().peekable();

        let result = parser.parse_natural_transformation(&mut chars, &mut context);
        assert!(result.is_ok());

        let expr = result.unwrap();
        assert!(matches!(
            expr,
            EnhancedMathExpression::UnicodeOperator { .. }
        ));
    }

    #[test]
    fn test_category_name_parsing() {
        let parser = CategoryTheoryParser::new();
        let mut context = create_test_context();
        let mut chars = "Set".chars().peekable();

        let result = parser.parse_category_name(&mut chars, &mut context);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Set");
    }

    #[test]
    fn test_mathematical_letter_detection() {
        let parser = CategoryTheoryParser::new();

        assert!(parser.is_mathematical_letter('𝔸')); // Mathematical bold A
        assert!(parser.is_mathematical_letter('ℂ')); // Complex numbers
        assert!(!parser.is_mathematical_letter('a')); // Regular letter
    }

    #[test]
    fn test_category_registry_lookup() {
        let parser = CategoryTheoryParser::new();

        let composition_info = parser.category_registry.get("∘");
        assert!(composition_info.is_some());
        assert_eq!(composition_info.unwrap().meaning, "Morphism composition");

        let unknown_symbol = parser.category_registry.get("unknown");
        assert!(unknown_symbol.is_none());
    }

    #[test]
    fn test_construct_creation() {
        let parser = CategoryTheoryParser::new();
        let symbol_info = CategorySymbolInfo::new(
            "∘".to_string(),
            "composition".to_string(),
            "category".to_string(),
        );

        let construct = parser.create_construct_from_symbol("∘", &symbol_info);
        assert!(matches!(construct, CategoryConstruct::Composition { .. }));
    }

    #[test]
    fn test_category_component_parsing() {
        let parser = CategoryTheoryParser::new();
        let mut context = create_test_context();
        let mut chars = "Objects,".chars().peekable();

        let result = parser.parse_category_component(&mut chars, &mut context);
        assert!(result.is_ok());

        let expr = result.unwrap();
        assert!(matches!(expr, EnhancedMathExpression::BasicSymbol(_)));
    }
}
