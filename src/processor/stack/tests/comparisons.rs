use crate::math::{ F128, FiniteField };
use super::{ init_stack, get_stack_state, get_aux_state, ExecutionHint, TRACE_LENGTH };
use super::super::Stack;

// EQUALITY OPERATION
// ================================================================================================

#[test]
fn eq() {
    let mut stack = init_stack(&[3, 3, 4, 5], &[], &[], TRACE_LENGTH);

    stack.op_eq(0);
    assert_eq!(vec![1], get_aux_state(&stack, 0));
    assert_eq!(vec![1, 4, 5, 0, 0, 0, 0, 0], get_stack_state(&stack, 1));

    assert_eq!(3, stack.depth);
    assert_eq!(4, stack.max_depth);

    stack.op_eq(1);
    let inv_diff = F128::inv(F128::sub(1, 4));
    assert_eq!(vec![inv_diff], get_aux_state(&stack, 1));
    assert_eq!(vec![0, 5, 0, 0, 0, 0, 0, 0], get_stack_state(&stack, 2));

    assert_eq!(2, stack.depth);
    assert_eq!(4, stack.max_depth);
}

// COMPARISON OPERATION
// ================================================================================================

#[test]
fn cmp_128() {

    let a: u128 = F128::rand();
    let b: u128 = F128::rand();
    let p127: u128 = F128::exp(2, 127);
    
    // initialize the stack
    let (inputs_a, inputs_b) = build_inputs_for_cmp(a, b, 128);
    let mut stack = init_stack(&[0, 0, 0, 0, 0, 0, a, b], &inputs_a, &inputs_b, 256);
    stack.op_push(0, p127);

    // execute CMP operations
    for i in 1..129 {
        stack.op_cmp(i, ExecutionHint::None);

        let state = get_stack_state(&stack, i);

        let gt = state[3];
        let lt = state[4];
        let not_set = F128::mul(F128::sub(F128::ONE, gt), F128::sub(F128::ONE, lt));
        assert_eq!(vec![not_set], get_aux_state(&stack, i));
    }

    // check the result
    let lt = if a < b { F128::ONE }  else { F128::ZERO };
    let gt = if a < b { F128::ZERO } else { F128::ONE  };

    let state = get_stack_state(&stack, 129);
    assert_eq!([gt, lt, b, a], state[3..7]);
}

#[test]
fn cmp_64() {

    let a: u128 = (F128::rand() as u64) as u128;
    let b: u128 = (F128::rand() as u64) as u128;
    let p63: u128 = F128::exp(2, 63);
    
    // initialize the stack
    let (inputs_a, inputs_b) = build_inputs_for_cmp(a, b, 64);
    let mut stack = init_stack(&[0, 0, 0, 0, 0, 0, a, b], &inputs_a, &inputs_b, 256);
    stack.op_push(0, p63);

    // execute CMP operations
    for i in 1..65 {
        stack.op_cmp(i, ExecutionHint::None);

        let state = get_stack_state(&stack, i);

        let gt = state[3];
        let lt = state[4];
        let not_set = F128::mul(F128::sub(F128::ONE, gt), F128::sub(F128::ONE, lt));
        assert_eq!(vec![not_set], get_aux_state(&stack, i));
    }

    // check the result
    let lt = if a < b { F128::ONE }  else { F128::ZERO };
    let gt = if a < b { F128::ZERO } else { F128::ONE  };

    let state = get_stack_state(&stack, 65);
    assert_eq!([gt, lt, b, a], state[3..7]);
}

// COMPARISON PROGRAMS
// ================================================================================================

#[test]
fn lt() {

    let a: u128 = F128::rand();
    let b: u128 = F128::rand();
    let p127: u128 = F128::exp(2, 127);
    
    // initialize the stack
    let (inputs_a, inputs_b) = build_inputs_for_cmp(a, b, 128);
    let mut stack = init_stack(&[0, 0, 0, 0, a, b, 7, 11], &inputs_a, &inputs_b, 256);
    stack.op_pad2(0);
    stack.op_push(1, p127);

    // execute CMP operations
    for i in 2..130 { stack.op_cmp(i, ExecutionHint::None); }

    // execute program finale
    let step = lt_finale(&mut stack, 130);

    // check the result
    let state = get_stack_state(&stack, step);
    let expected = if a < b { F128::ONE }  else { F128::ZERO };
    assert_eq!(vec![expected, 7, 11, 0, 0, 0, 0, 0, 0, 0, 0], state);
}

#[test]
fn gt() {

    let a: u128 = F128::rand();
    let b: u128 = F128::rand();
    let p127: u128 = F128::exp(2, 127);
    
    // initialize the stack
    let (inputs_a, inputs_b) = build_inputs_for_cmp(a, b, 128);
    let mut stack = init_stack(&[0, 0, 0, 0, a, b, 7, 11], &inputs_a, &inputs_b, 256);
    stack.op_pad2(0);
    stack.op_push(1, p127);

    // execute CMP operations
    for i in 2..130 { stack.op_cmp(i, ExecutionHint::None); }

    // execute program finale
    let step = gt_finale(&mut stack, 130);

    // check the result
    let state = get_stack_state(&stack, step);
    let expected = if a > b { F128::ONE }  else { F128::ZERO };
    assert_eq!(vec![expected, 7, 11, 0, 0, 0, 0, 0, 0, 0, 0], state);
}

// BINARY DECOMPOSITION
// ================================================================================================

#[test]
fn binacc_128() {

    let x: u128 = F128::rand();
    let p127: u128 = F128::exp(2, 127);
    
    // initialize the stack
    let mut inputs_a = Vec::new();
    for i in 0..128 { inputs_a.push((x >> i) & 1); }
    inputs_a.reverse();

    let mut stack = init_stack(&[p127, 0, x, 7, 11], &inputs_a, &[], 256);

    // execute binary aggregation operations
    for i in 0..128 { stack.op_binacc(i, ExecutionHint::None); }

    // check the result
    stack.op_drop(128);
    let state = get_stack_state(&stack, 129);
    assert_eq!(vec![x, x, 7, 11, 0, 0, 0, 0], state);
}

#[test]
fn binacc_64() {

    let x: u128 = (F128::rand() as u64) as u128;
    let p127: u128 = F128::exp(2, 63);
    
    // initialize the stack
    let mut inputs_a = Vec::new();
    for i in 0..64 { inputs_a.push((x >> i) & 1); }
    inputs_a.reverse();

    let mut stack = init_stack(&[p127, 0, x, 7, 11], &inputs_a, &[], 256);

    // execute binary aggregation operations
    for i in 0..64 { stack.op_binacc(i, ExecutionHint::None); }

    // check the result
    stack.op_drop(64);
    let state = get_stack_state(&stack, 65);
    assert_eq!(vec![x, x, 7, 11, 0, 0, 0, 0], state);
}

// HELPER FUNCTIONS
// ================================================================================================
fn build_inputs_for_cmp(a: u128, b: u128, size: usize) -> (Vec<u128>, Vec<u128>) {

    let mut inputs_a = Vec::new();
    let mut inputs_b = Vec::new();
    for i in 0..size {
        inputs_a.push((a >> i) & 1);
        inputs_b.push((b >> i) & 1);
    }
    inputs_a.reverse();
    inputs_b.reverse();

    return (inputs_a, inputs_b);
}

fn lt_finale(stack: &mut Stack, step: usize) -> usize {
    stack.op_drop(step + 0);
    stack.op_swap4(step + 1);
    stack.op_roll4(step + 2);
    stack.op_eq(step + 3);
    stack.op_assert(step + 4);
    stack.op_eq(step + 5);
    stack.op_assert(step + 6);
    stack.op_drop(step + 7);
    stack.op_drop(step + 8);
    stack.op_drop(step + 9);
    return step + 10;
}

fn gt_finale(stack: &mut Stack, step: usize) -> usize {
    stack.op_drop(step + 0);
    stack.op_swap4(step + 1);
    stack.op_roll4(step + 2);
    stack.op_eq(step + 3);
    stack.op_assert(step + 4);
    stack.op_eq(step + 5);
    stack.op_assert(step + 6);
    stack.op_drop(step + 7);
    stack.op_drop(step + 8);
    stack.op_swap(step + 9);
    stack.op_drop(step + 10);
    return step + 11;
}