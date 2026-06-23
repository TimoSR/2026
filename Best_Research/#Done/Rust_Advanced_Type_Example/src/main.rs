mod grading_students
{
    use std::fmt;

    // public primitive boundary types
    pub type RawGrade = i32;
    pub type RawGrades = [RawGrade];
    pub type RawRoundedGrades = Vec<RawGrade>;
    pub type GradingResult<T> = Result<T, GradeError>;
    // public primitive boundary types

    // data structures
    pub struct GradingStudents;

    struct FailingRule;
    struct PassingRule;
    struct MultipleOfFiveRule;
    struct NotMultipleOfFiveRule;
    struct ShouldBeRoundedRule;
    struct DefaultRoundingPolicy;
    // data structures

    // private strong domain types
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    struct Grade(RawGrade);

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    struct Remainder(RawGrade);

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    struct Delta(RawGrade);
    // private strong domain types

    // private type aliases
    type Grades = [Grade];
    type RoundedGrades = Vec<Grade>;

    type GradeRule = fn(Grade) -> bool;
    type GradeOperation = fn(Grade, Grade) -> GradingResult<Grade>;
    // private type aliases

    // public enums
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum GradeError
    {
        BelowMinimum
        {
            value: RawGrade
        },
        AboveMaximum
        {
            value: RawGrade
        },
        ArithmeticOverflow
        {
            left: RawGrade, right: RawGrade
        },
    }
    // public enums

    // private enums
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum RoundingDecision
    {
        Keep(Grade),
        Round
        {
            original: Grade,
            delta: Delta,
        },
    }
    // private enums

    // traits
    trait DomainScalar
    {
        fn value(self) -> RawGrade;
    }

    trait TryCreateFromRawGrade: Sized
    {
        fn try_create(value: RawGrade) -> GradingResult<Self>;
    }

    trait GradePredicate
    {
        fn name(&self) -> &'static str;

        fn evaluate(&self, grade: Grade) -> bool;
    }

    trait RoundingPolicy
    {
        fn round_grade(&self, grade: Grade) -> GradingResult<Grade>;
    }
    // traits

    // domain constants
    const MIN_GRADE: RawGrade = 0;
    const MAX_GRADE: RawGrade = 100;

    const MIN_PASSING_GRADE: Grade = Grade(38);
    const ROUNDING_BASE: Grade = Grade(5);
    const ROUNDING_DELTA_LIMIT: Delta = Delta(3);
    // domain constants

    // private domain language with function-pointer constants
    const IS_FAILING: GradeRule = |grade: Grade| grade.value() < MIN_PASSING_GRADE.value();

    const IS_PASSING: GradeRule = |grade: Grade| !IS_FAILING(grade);

    const IS_MULTIPLE_OF_FIVE: GradeRule = |grade: Grade| grade.value() % ROUNDING_BASE.value() == 0;

    const IS_NOT_MULTIPLE_OF_FIVE: GradeRule = |grade: Grade| !IS_MULTIPLE_OF_FIVE(grade);

    const GRADE_SHOULD_BE_ROUNDED: GradeRule = |grade: Grade| IS_PASSING(grade) && IS_NOT_MULTIPLE_OF_FIVE(grade);
    // private domain language with function-pointer constants

    impl fmt::Display for GradeError
    {
        fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result
        {
            match self
            {
                Self::BelowMinimum { value } =>
                {
                    write!(formatter, "grade must not be below {MIN_GRADE}: {value}")
                }
                Self::AboveMaximum { value } =>
                {
                    write!(formatter, "grade must not be above {MAX_GRADE}: {value}")
                }
                Self::ArithmeticOverflow { left, right } =>
                {
                    write!(formatter, "grade arithmetic overflow: {left} + {right}")
                }
            }
        }
    }

    impl DomainScalar for Grade
    {
        fn value(self) -> RawGrade
        {
            self.0
        }
    }

    impl DomainScalar for Remainder
    {
        fn value(self) -> RawGrade
        {
            self.0
        }
    }

    impl DomainScalar for Delta
    {
        fn value(self) -> RawGrade
        {
            self.0
        }
    }

    impl TryCreateFromRawGrade for Grade
    {
        fn try_create(value: RawGrade) -> GradingResult<Self>
        {
            if value < MIN_GRADE
            {
                return Err(GradeError::BelowMinimum { value });
            }

            if value > MAX_GRADE
            {
                return Err(GradeError::AboveMaximum { value });
            }

            Ok(Self(value))
        }
    }

    impl From<Grade> for RawGrade
    {
        fn from(grade: Grade) -> Self
        {
            grade.value()
        }
    }

    impl Grade
    {
        fn new(value: RawGrade) -> GradingResult<Self>
        {
            Self::try_create(value)
        }

        fn remainder_by(self, base: Grade) -> Remainder
        {
            Remainder(self.value() % base.value())
        }

        fn add_delta(self, delta: Delta) -> GradingResult<Grade>
        {
            let value = self.value().checked_add(delta.value()).ok_or(GradeError::ArithmeticOverflow {
                left: self.value(),
                right: delta.value(),
            })?;

            Grade::new(value)
        }

        fn percentage_as_f64(self) -> f64
        {
            let numerator = self.value() as f64;
            let denominator = MAX_GRADE as f64;

            numerator / denominator
        }

        fn report_value_as_i64(self) -> i64
        {
            let value: RawGrade = self.into();

            i64::from(value)
        }
    }

    impl Delta
    {
        fn new(value: RawGrade) -> Self
        {
            Self(value)
        }

        fn is_below(self, limit: Delta) -> bool
        {
            self.value() < limit.value()
        }
    }

    impl RoundingDecision
    {
        fn into_grade(self) -> GradingResult<Grade>
        {
            match self
            {
                Self::Keep(grade) => Ok(grade),
                Self::Round { original, delta } => original.add_delta(delta),
            }
        }
    }

    impl GradePredicate for FailingRule
    {
        fn name(&self) -> &'static str
        {
            "is_failing"
        }

        fn evaluate(&self, grade: Grade) -> bool
        {
            grade.value() < MIN_PASSING_GRADE.value()
        }
    }

    impl GradePredicate for PassingRule
    {
        fn name(&self) -> &'static str
        {
            "is_passing"
        }

        fn evaluate(&self, grade: Grade) -> bool
        {
            !FailingRule.evaluate(grade)
        }
    }

    impl GradePredicate for MultipleOfFiveRule
    {
        fn name(&self) -> &'static str
        {
            "is_multiple_of_five"
        }

        fn evaluate(&self, grade: Grade) -> bool
        {
            grade.value() % ROUNDING_BASE.value() == 0
        }
    }

    impl GradePredicate for NotMultipleOfFiveRule
    {
        fn name(&self) -> &'static str
        {
            "is_not_multiple_of_five"
        }

        fn evaluate(&self, grade: Grade) -> bool
        {
            !MultipleOfFiveRule.evaluate(grade)
        }
    }

    impl GradePredicate for ShouldBeRoundedRule
    {
        fn name(&self) -> &'static str
        {
            "grade_should_be_rounded"
        }

        fn evaluate(&self, grade: Grade) -> bool
        {
            PassingRule.evaluate(grade) && NotMultipleOfFiveRule.evaluate(grade)
        }
    }

    impl DefaultRoundingPolicy
    {
        fn decide(grade: Grade) -> RoundingDecision
        {
            if !ShouldBeRoundedRule.evaluate(grade)
            {
                return RoundingDecision::Keep(grade);
            }

            let remainder: Remainder = grade.remainder_by(ROUNDING_BASE);
            let delta: Delta = Delta::new(ROUNDING_BASE.value() - remainder.value());

            if delta.is_below(ROUNDING_DELTA_LIMIT)
            {
                return RoundingDecision::Round { original: grade, delta };
            }

            RoundingDecision::Keep(grade)
        }
    }

    impl RoundingPolicy for DefaultRoundingPolicy
    {
        fn round_grade(&self, grade: Grade) -> GradingResult<Grade>
        {
            Self::decide(grade).into_grade()
        }
    }

    impl GradingStudents
    {
        // private operations
        const ADD_DELEGATE: GradeOperation = |a: Grade, b: Grade| {
            let value = a.value().checked_add(b.value()).ok_or(GradeError::ArithmeticOverflow {
                left: a.value(),
                right: b.value(),
            })?;

            Grade::new(value)
        };
        // private operations

        // public API
        pub fn grading_students_method_with_closure_consts(grades: &RawGrades) -> GradingResult<RawRoundedGrades>
        {
            let grades = Self::parse_grades(grades)?;
            let rounded_grades = Self::grading_students_method_with_closure_consts_internal(&grades)?;

            Ok(Self::serialize_grades(&rounded_grades))
        }

        pub fn grading_students_method_with_lambdas(grades: &RawGrades) -> GradingResult<RawRoundedGrades>
        {
            let grades = Self::parse_grades(grades)?;
            let rounded_grades = Self::grading_students_method_with_lambdas_internal(&grades)?;

            Ok(Self::serialize_grades(&rounded_grades))
        }

        pub fn grading_students_method_with_traits(grades: &RawGrades) -> GradingResult<RawRoundedGrades>
        {
            let grades = Self::parse_grades(grades)?;
            let rounded_grades = Self::grading_students_method_with_traits_internal(&grades)?;

            Ok(Self::serialize_grades(&rounded_grades))
        }

        pub fn add_for_public_demo(a: RawGrade, b: RawGrade) -> GradingResult<RawGrade>
        {
            let left = Grade::new(a)?;
            let right = Grade::new(b)?;

            let result = Self::add_for_internal_use(left, right)?;

            Ok(result.into())
        }
        // public API

        // private conversion boundary
        fn parse_grades(values: &RawGrades) -> GradingResult<Vec<Grade>>
        {
            let mut grades = Vec::with_capacity(values.len());

            for value in values
            {
                let grade = Grade::new(*value)?;

                grades.push(grade);
            }

            Ok(grades)
        }

        fn serialize_grades(grades: &Grades) -> RawRoundedGrades
        {
            let mut values = Vec::with_capacity(grades.len());

            for grade in grades
            {
                let value: RawGrade = (*grade).into();

                values.push(value);
            }

            values
        }
        // private conversion boundary

        // private implementations
        fn grading_students_method_with_closure_consts_internal(grades: &Grades) -> GradingResult<RoundedGrades>
        {
            let mut rounded_grades = Vec::with_capacity(grades.len());

            for grade in grades
            {
                let mut result = *grade;

                if GRADE_SHOULD_BE_ROUNDED(result)
                {
                    let remainder: Remainder = result.remainder_by(ROUNDING_BASE);
                    let delta: Delta = Delta::new(ROUNDING_BASE.value() - remainder.value());

                    if delta.is_below(ROUNDING_DELTA_LIMIT)
                    {
                        result = result.add_delta(delta)?;
                    }
                }

                rounded_grades.push(result);
            }

            Ok(rounded_grades)
        }

        fn grading_students_method_with_lambdas_internal(grades: &Grades) -> GradingResult<RoundedGrades>
        {
            let is_failing = |grade: Grade| grade.value() < MIN_PASSING_GRADE.value();
            let is_passing = |grade: Grade| !is_failing(grade);
            let is_multiple_of_five = |grade: Grade| grade.value() % ROUNDING_BASE.value() == 0;
            let is_not_multiple_of_five = |grade: Grade| !is_multiple_of_five(grade);

            let grade_should_be_rounded = |grade: Grade| is_passing(grade) && is_not_multiple_of_five(grade);

            let is_failing_delegate_example = |grade: Grade| grade.value() < MIN_PASSING_GRADE.value();

            let _ = is_failing_delegate_example;

            let mut rounded_grades = Vec::with_capacity(grades.len());

            for grade in grades
            {
                let mut result = *grade;

                if grade_should_be_rounded(result)
                {
                    let remainder: Remainder = result.remainder_by(ROUNDING_BASE);
                    let delta: Delta = Delta::new(ROUNDING_BASE.value() - remainder.value());

                    if delta.is_below(ROUNDING_DELTA_LIMIT)
                    {
                        result = result.add_delta(delta)?;
                    }
                }

                rounded_grades.push(result);
            }

            Ok(rounded_grades)
        }

        fn grading_students_method_with_traits_internal(grades: &Grades) -> GradingResult<RoundedGrades>
        {
            let policy = DefaultRoundingPolicy;
            let mut rounded_grades = Vec::with_capacity(grades.len());

            for grade in grades
            {
                let rounded_grade = policy.round_grade(*grade)?;

                rounded_grades.push(rounded_grade);
            }

            Ok(rounded_grades)
        }

        fn add_for_internal_use(a: Grade, b: Grade) -> GradingResult<Grade>
        {
            Self::ADD_DELEGATE(a, b)
        }

        fn all_trait_rules_pass(grade: Grade, rules: &[&dyn GradePredicate]) -> bool
        {
            rules.iter().all(|rule| rule.evaluate(grade))
        }
        // private implementations
    }

    #[cfg(test)]
    mod tests
    {
        use super::*;

        #[test]
        fn grades_are_rounded_correctly_with_closure_consts()
        {
            let grades = vec![73, 67, 38, 33];

            let rounded_grades = GradingStudents::grading_students_method_with_closure_consts(&grades);

            assert_eq!(rounded_grades, Ok(vec![75, 67, 40, 33]));
        }

        #[test]
        fn grades_are_rounded_correctly_with_lambdas()
        {
            let grades = vec![73, 67, 38, 33];

            let rounded_grades = GradingStudents::grading_students_method_with_lambdas(&grades);

            assert_eq!(rounded_grades, Ok(vec![75, 67, 40, 33]));
        }

        #[test]
        fn grades_are_rounded_correctly_with_traits()
        {
            let grades = vec![73, 67, 38, 33];

            let rounded_grades = GradingStudents::grading_students_method_with_traits(&grades);

            assert_eq!(rounded_grades, Ok(vec![75, 67, 40, 33]));
        }

        #[test]
        fn invalid_negative_grade_is_rejected()
        {
            let grades = vec![73, -1, 38, 33];

            let rounded_grades = GradingStudents::grading_students_method_with_closure_consts(&grades);

            assert_eq!(rounded_grades, Err(GradeError::BelowMinimum { value: -1 }));
        }

        #[test]
        fn invalid_above_maximum_grade_is_rejected()
        {
            let grades = vec![73, 101, 38, 33];

            let rounded_grades = GradingStudents::grading_students_method_with_closure_consts(&grades);

            assert_eq!(rounded_grades, Err(GradeError::AboveMaximum { value: 101 }));
        }

        #[test]
        fn private_internal_closure_consts_implementation_is_testable() -> GradingResult<()>
        {
            let grades = vec![Grade::new(73)?, Grade::new(67)?, Grade::new(38)?, Grade::new(33)?];

            let rounded_grades = GradingStudents::grading_students_method_with_closure_consts_internal(&grades);

            assert_eq!(
                rounded_grades,
                Ok(vec![Grade::new(75)?, Grade::new(67)?, Grade::new(40)?, Grade::new(33)?,])
            );

            Ok(())
        }

        #[test]
        fn private_internal_lambdas_implementation_is_testable() -> GradingResult<()>
        {
            let grades = vec![Grade::new(73)?, Grade::new(67)?, Grade::new(38)?, Grade::new(33)?];

            let rounded_grades = GradingStudents::grading_students_method_with_lambdas_internal(&grades);

            assert_eq!(
                rounded_grades,
                Ok(vec![Grade::new(75)?, Grade::new(67)?, Grade::new(40)?, Grade::new(33)?,])
            );

            Ok(())
        }

        #[test]
        fn private_internal_traits_implementation_is_testable() -> GradingResult<()>
        {
            let grades = vec![Grade::new(73)?, Grade::new(67)?, Grade::new(38)?, Grade::new(33)?];

            let rounded_grades = GradingStudents::grading_students_method_with_traits_internal(&grades);

            assert_eq!(
                rounded_grades,
                Ok(vec![Grade::new(75)?, Grade::new(67)?, Grade::new(40)?, Grade::new(33)?,])
            );

            Ok(())
        }

        #[test]
        fn failing_rule_works() -> GradingResult<()>
        {
            assert!(IS_FAILING(Grade::new(37)?));
            assert!(!IS_FAILING(Grade::new(38)?));

            Ok(())
        }

        #[test]
        fn passing_rule_is_composed_from_failing_rule() -> GradingResult<()>
        {
            assert!(!IS_PASSING(Grade::new(37)?));
            assert!(IS_PASSING(Grade::new(38)?));

            Ok(())
        }

        #[test]
        fn multiple_of_five_rule_works() -> GradingResult<()>
        {
            assert!(IS_MULTIPLE_OF_FIVE(Grade::new(40)?));
            assert!(!IS_MULTIPLE_OF_FIVE(Grade::new(41)?));

            Ok(())
        }

        #[test]
        fn not_multiple_of_five_rule_is_composed_from_multiple_rule() -> GradingResult<()>
        {
            assert!(!IS_NOT_MULTIPLE_OF_FIVE(Grade::new(40)?));
            assert!(IS_NOT_MULTIPLE_OF_FIVE(Grade::new(41)?));

            Ok(())
        }

        #[test]
        fn grade_should_be_rounded_rule_is_composed_from_smaller_rules() -> GradingResult<()>
        {
            assert!(GRADE_SHOULD_BE_ROUNDED(Grade::new(38)?));
            assert!(GRADE_SHOULD_BE_ROUNDED(Grade::new(39)?));
            assert!(!GRADE_SHOULD_BE_ROUNDED(Grade::new(37)?));
            assert!(!GRADE_SHOULD_BE_ROUNDED(Grade::new(40)?));

            Ok(())
        }

        #[test]
        fn trait_rules_work() -> GradingResult<()>
        {
            let failing_rule = FailingRule;
            let passing_rule = PassingRule;
            let multiple_of_five_rule = MultipleOfFiveRule;
            let not_multiple_of_five_rule = NotMultipleOfFiveRule;
            let should_be_rounded_rule = ShouldBeRoundedRule;

            assert_eq!(failing_rule.name(), "is_failing");
            assert!(failing_rule.evaluate(Grade::new(37)?));

            assert_eq!(passing_rule.name(), "is_passing");
            assert!(passing_rule.evaluate(Grade::new(38)?));

            assert_eq!(multiple_of_five_rule.name(), "is_multiple_of_five");
            assert!(multiple_of_five_rule.evaluate(Grade::new(40)?));

            assert_eq!(not_multiple_of_five_rule.name(), "is_not_multiple_of_five");
            assert!(not_multiple_of_five_rule.evaluate(Grade::new(41)?));

            assert_eq!(should_be_rounded_rule.name(), "grade_should_be_rounded");
            assert!(should_be_rounded_rule.evaluate(Grade::new(39)?));

            Ok(())
        }

        #[test]
        fn trait_objects_can_compose_rules_dynamically() -> GradingResult<()>
        {
            let grade = Grade::new(39)?;

            let passing_rule = PassingRule;
            let not_multiple_of_five_rule = NotMultipleOfFiveRule;

            let rules: [&dyn GradePredicate; 2] = [&passing_rule, &not_multiple_of_five_rule];

            assert!(GradingStudents::all_trait_rules_pass(grade, &rules));

            Ok(())
        }

        #[test]
        fn rounding_policy_uses_enum_decision() -> GradingResult<()>
        {
            let round_decision = DefaultRoundingPolicy::decide(Grade::new(39)?);
            let keep_decision = DefaultRoundingPolicy::decide(Grade::new(40)?);

            assert_eq!(
                round_decision,
                RoundingDecision::Round {
                    original: Grade::new(39)?,
                    delta: Delta::new(1),
                }
            );

            assert_eq!(keep_decision, RoundingDecision::Keep(Grade::new(40)?));

            assert_eq!(round_decision.into_grade(), Ok(Grade::new(40)?));
            assert_eq!(keep_decision.into_grade(), Ok(Grade::new(40)?));

            Ok(())
        }

        #[test]
        fn private_add_delegate_works_through_impl_private_const() -> GradingResult<()>
        {
            let result = GradingStudents::ADD_DELEGATE(Grade::new(2)?, Grade::new(3)?);

            assert_eq!(result, Ok(Grade::new(5)?));

            Ok(())
        }

        #[test]
        fn private_add_wrapper_works() -> GradingResult<()>
        {
            let result = GradingStudents::add_for_internal_use(Grade::new(2)?, Grade::new(3)?);

            assert_eq!(result, Ok(Grade::new(5)?));

            Ok(())
        }

        #[test]
        fn public_add_wrapper_works()
        {
            let result = GradingStudents::add_for_public_demo(2, 3);

            assert_eq!(result, Ok(5));
        }

        #[test]
        fn private_grade_is_a_real_type_not_an_i32_alias() -> GradingResult<()>
        {
            let grade = Grade::new(38)?;

            assert_eq!(grade.value(), 38);
            assert!(IS_PASSING(grade));

            Ok(())
        }

        #[test]
        fn private_types_are_still_testable_inside_module_tests() -> GradingResult<()>
        {
            let grade = Grade::new(38)?;

            let grades = vec![Grade::new(73)?, Grade::new(67)?, Grade::new(38)?, Grade::new(33)?];

            let grades_slice: &Grades = &grades;

            let rounded_grades: GradingResult<RoundedGrades> = GradingStudents::grading_students_method_with_closure_consts_internal(grades_slice);

            let remainder: Remainder = grade.remainder_by(ROUNDING_BASE);
            let delta: Delta = Delta::new(ROUNDING_BASE.value() - remainder.value());

            let rule: GradeRule = IS_PASSING;
            let operation: GradeOperation = GradingStudents::ADD_DELEGATE;

            assert_eq!(grade, MIN_PASSING_GRADE);
            assert_eq!(grades_slice.len(), 4);

            assert_eq!(
                rounded_grades,
                Ok(vec![Grade::new(75)?, Grade::new(67)?, Grade::new(40)?, Grade::new(33)?,])
            );

            assert_eq!(remainder, Remainder(3));
            assert_eq!(delta, Delta(2));
            assert_eq!(ROUNDING_DELTA_LIMIT, Delta(3));
            assert!(rule(grade));

            assert_eq!(operation(Grade::new(2)?, Grade::new(3)?), Ok(Grade::new(5)?));

            Ok(())
        }

        #[test]
        fn checked_type_conversion_from_raw_grade_to_grade_works()
        {
            let grade = Grade::try_create(38);

            assert_eq!(grade, Ok(Grade(38)));
        }

        #[test]
        fn checked_type_conversion_from_raw_grade_to_grade_rejects_invalid_values()
        {
            assert_eq!(Grade::try_create(-1), Err(GradeError::BelowMinimum { value: -1 }));

            assert_eq!(Grade::try_create(101), Err(GradeError::AboveMaximum { value: 101 }));
        }

        #[test]
        fn infallible_conversion_from_grade_to_raw_grade_works() -> GradingResult<()>
        {
            let grade = Grade::new(38)?;

            let value: RawGrade = grade.into();

            assert_eq!(value, 38);

            Ok(())
        }

        #[test]
        fn primitive_casting_is_explicit() -> GradingResult<()>
        {
            let grade = Grade::new(75)?;

            let percentage = grade.percentage_as_f64();
            let report_value = grade.report_value_as_i64();

            assert_eq!(percentage, 0.75);
            assert_eq!(report_value, 75_i64);

            Ok(())
        }

        #[test]
        fn public_error_display_works()
        {
            let error = GradeError::BelowMinimum { value: -1 };

            assert_eq!(error.to_string(), "grade must not be below 0: -1");
        }
    }
}

use grading_students::GradingStudents;

fn main()
{
    let grades = vec![73, 67, 38, 33];

    let rounded_grades_with_closure_consts = GradingStudents::grading_students_method_with_closure_consts(&grades);

    println!("{rounded_grades_with_closure_consts:?}");

    let rounded_grades_with_lambdas = GradingStudents::grading_students_method_with_lambdas(&grades);

    println!("{rounded_grades_with_lambdas:?}");

    let rounded_grades_with_traits = GradingStudents::grading_students_method_with_traits(&grades);

    println!("{rounded_grades_with_traits:?}");

    let add_result = GradingStudents::add_for_public_demo(2, 3);

    println!("{add_result:?}");
}
