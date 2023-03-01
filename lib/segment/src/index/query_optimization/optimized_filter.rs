use crate::types::PointOffsetType;

pub type ConditionCheckerFn<'a> = Box<dyn Fn(PointOffsetType) -> bool + 'a>;

pub enum OptimizedCondition<'a> {
    Checker(ConditionCheckerFn<'a>),
    /// Nested filter
    Filter(OptimizedFilter<'a>),
}

pub struct OptimizedFilter<'a> {
    /// At least one of those conditions should match
    pub should: Option<Vec<OptimizedCondition<'a>>>,
    /// All conditions must match
    pub must: Option<Vec<OptimizedCondition<'a>>>,
    /// All conditions must NOT match
    pub must_not: Option<Vec<OptimizedCondition<'a>>>,
    /// Nested object filter
    pub nested: Option<Box<NestedOptimizedFilter<'a>>>,
}

pub struct NestedOptimizedFilter<'a> {
    pub path: &'a str,
    pub filter: OptimizedFilter<'a>,
}

pub fn check_optimized_filter(filter: &OptimizedFilter, point_id: PointOffsetType) -> bool {
    check_should(&filter.should, point_id)
        && check_must(&filter.must, point_id)
        && check_must_not(&filter.must_not, point_id)
        && check_nested(&filter.nested, point_id)
}

fn check_nested(nested: &Option<Box<NestedOptimizedFilter>>, point_id: PointOffsetType) -> bool {
    match nested {
        None => true,
        Some(nested) => {
            // TODO so far only one level of nesting is supported on `must`
            let _path = nested.path;
            nested.filter.must.as_ref().map_or(true, |must| {
                must.iter().any(|condition| match condition {
                    OptimizedCondition::Filter(_filter) => {
                        unreachable!("no nested filter in nested object filter");
                    }
                    OptimizedCondition::Checker(checker) => {
                        eprintln!("nested condition checker");
                        checker(point_id)
                    }
                })
            })
        }
    }
}

fn check_condition(condition: &OptimizedCondition, point_id: PointOffsetType) -> bool {
    match condition {
        OptimizedCondition::Filter(filter) => check_optimized_filter(filter, point_id),
        OptimizedCondition::Checker(checker) => checker(point_id),
    }
}

fn check_should(should: &Option<Vec<OptimizedCondition>>, point_id: PointOffsetType) -> bool {
    let check = |condition| check_condition(condition, point_id);
    match should {
        None => true,
        Some(conditions) => conditions.iter().any(check),
    }
}

fn check_must(must: &Option<Vec<OptimizedCondition>>, point_id: PointOffsetType) -> bool {
    let check = |condition| check_condition(condition, point_id);
    match must {
        None => true,
        Some(conditions) => conditions.iter().all(check),
    }
}

fn check_must_not(must: &Option<Vec<OptimizedCondition>>, point_id: PointOffsetType) -> bool {
    let check = |condition| !check_condition(condition, point_id);
    match must {
        None => true,
        Some(conditions) => conditions.iter().all(check),
    }
}
