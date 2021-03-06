//! Projection expression plan.

use timely::dataflow::Scope;
use timely::dataflow::scopes::child::Iterative;

use plan::Implementable;
use Relation;
use {QueryMap, RelationMap, SimpleRelation, Var};

/// A plan stage projecting its source to only the specified sequence
/// of symbols. Throws on unbound symbols. Frontends are responsible
/// for ensuring that the source binds all requested symbols.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Project<P: Implementable> {
    /// TODO
    pub variables: Vec<Var>,
    /// Plan for the data source.
    pub plan: Box<P>,
}

impl<P: Implementable> Implementable for Project<P> {
    fn implement<'b, S: Scope<Timestamp = u64>>(
        &self,
        nested: &mut Iterative<'b, S, u64>,
        local_arrangements: &RelationMap<Iterative<'b, S, u64>>,
        global_arrangements: &mut QueryMap<isize>,
    ) -> SimpleRelation<'b, S> {
        
        let relation = self.plan
            .implement(nested, local_arrangements, global_arrangements);
        let tuples = relation
            .tuples_by_symbols(&self.variables)
            .map(|(key, _tuple)| key);

        SimpleRelation {
            symbols: self.variables.to_vec(),
            tuples,
        }
    }
}
