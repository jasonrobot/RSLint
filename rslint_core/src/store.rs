//! A rule store, which houses rule groups as well as individual rules.

use crate::groups::*;
use crate::CstRule;

#[derive(Debug, Default, Clone)]
pub struct CstRuleStore {
    pub rules: Vec<Box<dyn CstRule>>,
}

impl CstRuleStore {
    pub fn new() -> Self {
        Self::default()
    }

    /// All built in rules from every group.
    pub fn builtins(mut self) -> Self {
        self.rules.extend(errors());
        self
    }

    pub fn load_rules(&mut self, rules: impl IntoIterator<Item = Box<dyn CstRule>>) {
        self.rules.extend(rules);
    }

    /// Get a rule using its rule name from this store.
    ///
    /// # Examples
    /// ```
    /// use rslint_core::CstRuleStore;
    ///
    /// assert!(CstRuleStore::new().builtins().get("no-empty").is_some())
    /// ```
    pub fn get(&self, rule_name: impl AsRef<str>) -> Option<Box<dyn CstRule>> {
        self.rules
            .iter()
            .find(|rule| rule.name() == rule_name.as_ref())
            .cloned()
    }
}
