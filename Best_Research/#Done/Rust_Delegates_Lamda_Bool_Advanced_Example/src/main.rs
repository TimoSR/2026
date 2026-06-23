mod grading_students
{
    // data structures
    pub struct GradingStudents;
    // data structures

    // private types
    type Grade = i32;
    type Grades = [Grade];
    type RoundedGrades = Vec<Grade>;

    type GradeRule = fn(Grade) -> bool;
    type GradeOperation = fn(Grade, Grade) -> Grade;
    type Remainder = Grade;
    type Delta = Grade;
    // private types

    // domain constants
    const MIN_PASSING_GRADE: Grade = 38;
    const ROUNDING_BASE: Grade = 5;
    const ROUNDING_DELTA_LIMIT: Delta = 3;
    // domain constants

    // private domain language
    const IS_FAILING: GradeRule = |grade: Grade| grade < MIN_PASSING_GRADE;

    const IS_PASSING: GradeRule = |grade: Grade| !IS_FAILING(grade);

    const IS_MULTIPLE_OF_FIVE: GradeRule = |grade: Grade| grade % ROUNDING_BASE == 0;

    const IS_NOT_MULTIPLE_OF_FIVE: GradeRule = |grade: Grade| !IS_MULTIPLE_OF_FIVE(grade);

    const GRADE_SHOULD_BE_ROUNDED: GradeRule = |grade: Grade| IS_PASSING(grade) && IS_NOT_MULTIPLE_OF_FIVE(grade);
    // private domain language

    impl GradingStudents
    {
        // the problem of including them within the impl is they require self, which makes reading worse.
        // boolean algebra-driven style
        // const IS_FAILING: fn(i32) -> bool = |grade: i32| grade < 38;

        // const IS_PASSING: fn(i32) -> bool = |grade: i32| !(Self::IS_FAILING)(grade);

        // const IS_MULTIPLE_OF_FIVE: fn(i32) -> bool = |grade: i32| grade % 5 == 0;

        // const IS_NOT_MULTIPLE_OF_FIVE: fn(i32) -> bool = |grade: i32| !(Self::IS_MULTIPLE_OF_FIVE)(grade);

        // const GRADE_SHOULD_BE_ROUNDED: fn(i32) -> bool = |grade: i32| (Self::IS_PASSING)(grade) && (Self::IS_NOT_MULTIPLE_OF_FIVE)(grade);

        const ADD_DELEGATE: GradeOperation = |a: Grade, b: Grade| {
            let result = a + b;
            result
        };

        pub fn grading_students_method_with_closure_consts(grades: &[i32]) -> Vec<i32>
        {
            Self::grading_students_method_with_closure_consts_internal(grades)
        }

        pub fn grading_students_method_with_lambdas(grades: &[i32]) -> Vec<i32>
        {
            Self::grading_students_method_with_lambdas_internal(grades)
        }

        pub fn add_for_public_demo(a: i32, b: i32) -> i32
        {
            Self::add_for_internal_use(a, b)
        }

        fn grading_students_method_with_closure_consts_internal(grades: &Grades) -> RoundedGrades
        {
            let mut rounded_grades = Vec::with_capacity(grades.len());

            for grade in grades
            {
                let mut result = *grade;

                if GRADE_SHOULD_BE_ROUNDED(result)
                {
                    let remainder: Remainder = result % ROUNDING_BASE;
                    let delta: Delta = ROUNDING_BASE - remainder;

                    if delta < ROUNDING_DELTA_LIMIT
                    {
                        result += delta;
                    }
                }

                rounded_grades.push(result);
            }

            rounded_grades
        }

        fn grading_students_method_with_lambdas_internal(grades: &Grades) -> RoundedGrades
        {
            let is_failing = |grade: Grade| grade < MIN_PASSING_GRADE;
            let is_passing = |grade: Grade| !is_failing(grade);
            let is_multiple_of_five = |grade: Grade| grade % ROUNDING_BASE == 0;
            let is_not_multiple_of_five = |grade: Grade| !is_multiple_of_five(grade);

            let grade_should_be_rounded = |grade: Grade| is_passing(grade) && is_not_multiple_of_five(grade);

            let is_failing_delegate_example = |grade: Grade| {
                if grade < 0
                {
                    panic!("grade must not be negative: {grade}");
                }

                grade < MIN_PASSING_GRADE
            };

            let _ = is_failing_delegate_example;

            let mut rounded_grades = Vec::with_capacity(grades.len());

            for grade in grades
            {
                let mut result = *grade;

                if grade_should_be_rounded(result)
                {
                    let remainder: Remainder = result % ROUNDING_BASE;
                    let delta: Delta = ROUNDING_BASE - remainder;

                    if delta < ROUNDING_DELTA_LIMIT
                    {
                        result += delta;
                    }
                }

                rounded_grades.push(result);
            }

            rounded_grades
        }

        fn add_for_internal_use(a: Grade, b: Grade) -> Grade
        {
            Self::ADD_DELEGATE(a, b)
        }
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

            assert_eq!(rounded_grades, vec![75, 67, 40, 33]);
        }

        #[test]
        fn grades_are_rounded_correctly_with_lambdas()
        {
            let grades = vec![73, 67, 38, 33];

            let rounded_grades = GradingStudents::grading_students_method_with_lambdas(&grades);

            assert_eq!(rounded_grades, vec![75, 67, 40, 33]);
        }

        #[test]
        fn private_internal_closure_consts_implementation_is_testable()
        {
            let grades = vec![73, 67, 38, 33];

            let rounded_grades = GradingStudents::grading_students_method_with_closure_consts_internal(&grades);

            assert_eq!(rounded_grades, vec![75, 67, 40, 33]);
        }

        #[test]
        fn private_internal_lambdas_implementation_is_testable()
        {
            let grades = vec![73, 67, 38, 33];

            let rounded_grades = GradingStudents::grading_students_method_with_lambdas_internal(&grades);

            assert_eq!(rounded_grades, vec![75, 67, 40, 33]);
        }

        #[test]
        fn failing_rule_works()
        {
            assert!(IS_FAILING(37));
            assert!(!IS_FAILING(38));
        }

        #[test]
        fn passing_rule_is_composed_from_failing_rule()
        {
            assert!(!IS_PASSING(37));
            assert!(IS_PASSING(38));
        }

        #[test]
        fn multiple_of_five_rule_works()
        {
            assert!(IS_MULTIPLE_OF_FIVE(40));
            assert!(!IS_MULTIPLE_OF_FIVE(41));
        }

        #[test]
        fn not_multiple_of_five_rule_is_composed_from_multiple_rule()
        {
            assert!(!IS_NOT_MULTIPLE_OF_FIVE(40));
            assert!(IS_NOT_MULTIPLE_OF_FIVE(41));
        }

        #[test]
        fn grade_should_be_rounded_rule_is_composed_from_smaller_rules()
        {
            assert!(GRADE_SHOULD_BE_ROUNDED(38));
            assert!(GRADE_SHOULD_BE_ROUNDED(39));
            assert!(!GRADE_SHOULD_BE_ROUNDED(37));
            assert!(!GRADE_SHOULD_BE_ROUNDED(40));
        }

        #[test]
        fn private_add_delegate_works_through_impl_private_const()
        {
            let result = GradingStudents::ADD_DELEGATE(2, 3);

            assert_eq!(result, 5);
        }

        #[test]
        fn private_add_wrapper_works()
        {
            let result = GradingStudents::add_for_internal_use(2, 3);

            assert_eq!(result, 5);
        }

        #[test]
        fn public_add_wrapper_works()
        {
            let result = GradingStudents::add_for_public_demo(2, 3);

            assert_eq!(result, 5);
        }

        #[test]
        fn private_grade_type_alias_is_still_usable_inside_tests()
        {
            let grade: Grade = 38;

            assert!(IS_PASSING(grade));
        }

        #[test]
        fn private_types_are_still_testable_inside_module_tests()
        {
            let grade: Grade = 38;
            let grades = vec![73, 67, 38, 33];
            let grades_slice: &Grades = &grades;
            let rounded_grades: RoundedGrades = GradingStudents::grading_students_method_with_closure_consts_internal(grades_slice);

            let remainder: Remainder = grade % ROUNDING_BASE;
            let delta: Delta = ROUNDING_BASE - remainder;

            let rule: GradeRule = IS_PASSING;
            let operation: GradeOperation = GradingStudents::ADD_DELEGATE;

            assert_eq!(grade, MIN_PASSING_GRADE);
            assert_eq!(grades_slice.len(), 4);
            assert_eq!(rounded_grades, vec![75, 67, 40, 33]);
            assert_eq!(remainder, 3);
            assert_eq!(delta, 2);
            assert_eq!(ROUNDING_DELTA_LIMIT, 3);
            assert!(rule(grade));
            assert_eq!(operation(2, 3), 5);
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

    let add_result = GradingStudents::add_for_public_demo(2, 3);

    println!("{add_result}");
}
