use std::fmt::Display;

use crate::word::Word;

//***************************************************************************
//
// ProductionRule
//
//***************************************************************************

pub struct ProductionRule<T> {
    lhs: Word<T>,
    rhs: Word<T>,
}

//***************************************************************************
//
// Impl ProductionRule
//
//***************************************************************************

impl<T> ProductionRule<T> {
    pub fn new(lhs: Word<T>, rhs: Word<T>) -> Self {
        Self {
            lhs,
            rhs,
        }
    }
    pub fn lhs(&self) -> &Word<T> {
        &self.lhs
    }
    pub fn rhs(&self) -> &Word<T> {
        &self.rhs
    }
}

impl<T> From<(&[T], &[T])> for ProductionRule<T>
where
    T: Clone,
{
    fn from(rule: (&[T], &[T])) -> Self {
        Self {
            lhs: Word::from(rule.0),
            rhs: Word::from(rule.1),
        }
    }
}

impl<T> From<&(&[T], &[T])> for ProductionRule<T>
where
    T: Clone,
{
    fn from(rule: &(&[T], &[T])) -> Self {
        Self {
            lhs: Word::from(rule.0),
            rhs: Word::from(rule.1),
        }
    }
}

// TODO: Maybe add data to the errors later, but then i need restrictions
// on the Data (the generic Type T) with regards to Debug and Display
#[derive(Debug, Clone)]
pub enum GrammarError {
    ProductionRulesContainUnknownTerminal,
    ProductionRulesContainUnknownNonTerminal,
}

impl Display for GrammarError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GrammarError::ProductionRulesContainUnknownTerminal => {
                write!(f, "production rules contain unknown terminals")
            }
            GrammarError::ProductionRulesContainUnknownNonTerminal => {
                write!(f, "production rules contain unknown non terminals")
            }
        }
    }
}

impl std::error::Error for GrammarError {}

pub struct Grammar<T>
where
    T: PartialEq,
{
    terminals: Vec<T>,
    non_terminals: Vec<T>,
    production_rules: Vec<ProductionRule<T>>,
}

impl<T> Grammar<T>
where
    T: PartialEq,
{
    pub fn new(terminals: Vec<T>, non_terminals: Vec<T>) -> Self {
        Self {
            terminals,
            non_terminals,
            production_rules: Vec::new(),
        }
    }
    pub fn validate_production_rules(&self, production_rules: &[ProductionRule<T>]) -> Result<(), GrammarError> {
        for (index, rule) in production_rules.iter().enumerate() {
            // check that neither the rhs nor the lhs contain symbols that are not contained in
            // terminals and nonterminals
            // TODO: If i decide to add data to the errors, i need to rework this section.
            if rule.lhs().contains_any_letter_besides(&self.terminals) || rule.rhs().contains_any_letter_besides(&self.terminals) {
                return Err(GrammarError::ProductionRulesContainUnknownTerminal);
            }
            if rule.lhs().contains_any_letter_besides(&self.non_terminals) || rule.rhs().contains_any_letter_besides(&self.non_terminals) {
                return Err(GrammarError::ProductionRulesContainUnknownNonTerminal);
            }
        }
        Ok(())
    }

    pub fn with_production_rules(self, production_rules: Vec<ProductionRule<T>>) -> Result<Self, GrammarError> {
        self.validate_production_rules(&production_rules).map(|()| Self { production_rules, ..self})
    }
}
