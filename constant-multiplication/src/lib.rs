// src/lib.rs
use pyo3::exceptions::{PyIndexError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::PyList;
use unsigned_varint::decode as varint_decode;
use std::collections::HashMap;

// Include the data files directly at compile time
const DATA_FILE: &[u8] = include_bytes!("../adder_cost.bin");
const GRAPH_TYPES_FILE: &[u8] = include_bytes!("../graph_types.bin");

// Parse the adder cost data at compile time
const fn parse_data_header() -> usize {
    // First 8 bytes are the count
    u64::from_le_bytes([
        DATA_FILE[0],
        DATA_FILE[1],
        DATA_FILE[2],
        DATA_FILE[3],
        DATA_FILE[4],
        DATA_FILE[5],
        DATA_FILE[6],
        DATA_FILE[7],
    ]) as usize
}

const DATA_COUNT: usize = parse_data_header();
const DATA_OFFSET: usize = 8;
const GRAPH_TYPES_BYTES: &[u8] = GRAPH_TYPES_FILE;

// Compile-time validation of DATA_FILE length
const _: () = assert!(DATA_FILE.len() >= DATA_OFFSET, "DATA_FILE is too small");

// GraphType as a Python class
#[pyclass]
#[derive(Clone)]
struct GraphType {
    #[pyo3(get)]
    variant: &'static str,
    #[pyo3(get)]
    params: Vec<usize>,
}

fn extract_shift(value: usize) -> String {
    let shift = value.trailing_zeros();
    if shift == 0 {
        value.to_string()
    } else {
        let shifted = value >> shift;
        format!("{shifted} << {shift}")
    }
}

#[pymethods]
impl GraphType {
    fn __repr__(&self) -> String {
        let shifted_params: Vec<String> = self.params.iter().map(|&v| extract_shift(v)).collect();
        format!("{}({})", self.variant, shifted_params.join(", "))
    }
}

/// Helper function to decode multiple usize parameters from varint-encoded data
fn decode_params(remaining: &mut &[u8], count: usize) -> Result<Vec<usize>, String> {
    let mut params = Vec::with_capacity(count);
    for _ in 0..count {
        let (param, rest) =
            varint_decode::usize(remaining).map_err(|e| format!("Failed to decode usize: {e}"))?;
        *remaining = rest;
        params.push(param);
    }
    Ok(params)
}

/// Deserialization with varint decoding
fn deserialize_graph_types(data: &[u8]) -> Result<Vec<Vec<GraphType>>, String> {
    let mut remaining = data;

    // Read number of entries
    let (count, rest) =
        varint_decode::usize(remaining).map_err(|e| format!("Failed to decode count: {e}"))?;
    remaining = rest;

    let mut result = Vec::with_capacity(count);

    for _ in 0..count {
        // Read length of this Vec
        let (vec_len, rest) = varint_decode::usize(remaining)
            .map_err(|e| format!("Failed to decode vec length: {e}"))?;
        remaining = rest;

        let mut type_vec = Vec::with_capacity(vec_len);

        for _ in 0..vec_len {
            if remaining.is_empty() {
                return Err("Unexpected end of data".to_string());
            }

            let variant_tag = remaining[0];
            remaining = &remaining[1..];

            let graph_type = match variant_tag {
                0 => {
                    let params = decode_params(&mut remaining, 2)?;
                    GraphType {
                        variant: "Adder",
                        params,
                    }
                }
                1 => {
                    let params = decode_params(&mut remaining, 2)?;
                    GraphType {
                        variant: "Subtractor",
                        params,
                    }
                }
                2 => {
                    let params = decode_params(&mut remaining, 2)?;
                    GraphType {
                        variant: "Cascade",
                        params,
                    }
                }
                3 => {
                    let params = decode_params(&mut remaining, 4)?;
                    GraphType {
                        variant: "Leapfrog4_1",
                        params,
                    }
                }
                4 => {
                    let params = decode_params(&mut remaining, 4)?;
                    GraphType {
                        variant: "Leapfrog4_2",
                        params,
                    }
                }
                5 => {
                    let params = decode_params(&mut remaining, 4)?;
                    GraphType {
                        variant: "Leapfrog4_3",
                        params,
                    }
                }
                6 => {
                    let params = decode_params(&mut remaining, 4)?;
                    GraphType {
                        variant: "Leapfrog4_4",
                        params,
                    }
                }
                7 => {
                    let params = decode_params(&mut remaining, 5)?;
                    GraphType {
                        variant: "Leapfrog5_1",
                        params,
                    }
                }
                8 => {
                    let params = decode_params(&mut remaining, 5)?;
                    GraphType {
                        variant: "Leapfrog5_2",
                        params,
                    }
                }
                9 => {
                    let params = decode_params(&mut remaining, 5)?;
                    GraphType {
                        variant: "Leapfrog5_3",
                        params,
                    }
                }
                10 => {
                    let params = decode_params(&mut remaining, 5)?;
                    GraphType {
                        variant: "Leapfrog5_4",
                        params,
                    }
                }
                11 => {
                    let params = decode_params(&mut remaining, 7)?;
                    GraphType {
                        variant: "Leapfrog7_1",
                        params,
                    }
                }
                12 => {
                    let params = decode_params(&mut remaining, 7)?;
                    GraphType {
                        variant: "Leapfrog7_2",
                        params,
                    }
                }
                13 => {
                    let params = decode_params(&mut remaining, 7)?;
                    GraphType {
                        variant: "Leapfrog7_3",
                        params,
                    }
                }
                14 => {
                    let params = decode_params(&mut remaining, 7)?;
                    GraphType {
                        variant: "Leapfrog7_4",
                        params,
                    }
                }
                15 => {
                    let params = decode_params(&mut remaining, 7)?;
                    GraphType {
                        variant: "Leapfrog7_5",
                        params,
                    }
                }
                16 => {
                    let params = decode_params(&mut remaining, 7)?;
                    GraphType {
                        variant: "Leapfrog7_6",
                        params,
                    }
                }
                17 => {
                    let params = decode_params(&mut remaining, 7)?;
                    GraphType {
                        variant: "Leapfrog7_7",
                        params,
                    }
                }
                18 => {
                    let params = decode_params(&mut remaining, 7)?;
                    GraphType {
                        variant: "Leapfrog7_8",
                        params,
                    }
                }
                _ => return Err(format!("Unknown variant tag: {variant_tag}")),
            };

            type_vec.push(graph_type);
        }

        result.push(type_vec);
    }

    Ok(result)
}

/// Get adder cost at index (right-shifts even indices until odd)
#[pyfunction]
fn adder_cost(mut idx: usize) -> PyResult<u8> {
    // Right-shift even indices until odd
    if idx > 0 {
        let shift = idx.trailing_zeros();
        idx >>= shift;
    }

    let value_position = idx / 2;
    if value_position >= DATA_COUNT {
        return Err(PyIndexError::new_err("Index out of range"));
    }

    let bit_offset = value_position * 3;
    let byte_offset = bit_offset / 8;
    let bit_in_byte = bit_offset % 8;

    if byte_offset >= DATA_FILE.len() - DATA_OFFSET {
        return Err(PyValueError::new_err("Data corruption"));
    }

    let mut val = (DATA_FILE[byte_offset + DATA_OFFSET] >> bit_in_byte) & 0b111;

    // Handle values that span two bytes
    if bit_in_byte > 5 && byte_offset + 1 < DATA_FILE.len() - DATA_OFFSET {
        let bits_from_next = 3 - (8 - bit_in_byte);
        val |= (DATA_FILE[byte_offset + 1 + DATA_OFFSET] & ((1 << bits_from_next) - 1))
            << (8 - bit_in_byte);
    }

    Ok(val & 0b111)
}

/// Get info about the embedded data
#[pyfunction]
fn info() -> String {
    format!(
        "Embedded data: {} elements, {} bytes packed, graph types: {} bytes compressed",
        DATA_COUNT * 2,
        DATA_FILE.len() - DATA_OFFSET,
        GRAPH_TYPES_BYTES.len()
    )
}

/// Get graph types at index (right-shifts even indices until odd)
#[pyfunction]
fn get_graph_types(py: Python, mut idx: usize) -> PyResult<Py<PyAny>> {
    // Right-shift even indices until odd
    if idx > 0 {
        let shift = idx.trailing_zeros();
        idx >>= shift;
    }

    let all_types = get_graph_types_data()?;

    // Convert odd index to position in the compact array
    // index 1 -> position 0, index 3 -> position 1, index 5 -> position 2, etc.
    let position = idx / 2;

    if position >= all_types.len() {
        return Err(PyIndexError::new_err("Index out of range"));
    }

    let types = &all_types[position];
    let list = PyList::empty(py);
    for gt in types {
        list.append(gt.clone())?;
    }
    Ok(list.into())
}

/// Get all graph types as a list
#[pyfunction]
fn get_all_graph_types(py: Python) -> PyResult<Py<PyAny>> {
    let all_types = get_graph_types_data()?;
    let result = PyList::empty(py);

    for types in all_types {
        let inner_list = PyList::empty(py);
        for gt in types {
            inner_list.append(gt)?;
        }
        result.append(inner_list)?;
    }

    Ok(result.into())
}

fn get_graph_types_data() -> PyResult<Vec<Vec<GraphType>>> {
    // Decompress the LZ4 data
    let decompressed = lz4_flex::decompress_size_prepended(GRAPH_TYPES_BYTES)
        .map_err(|e| PyValueError::new_err(format!("Failed to decompress: {e}")))?;

    // Deserialize with varint decoding
    deserialize_graph_types(&decompressed)
        .map_err(|e| PyValueError::new_err(format!("Failed to deserialize: {e}")))
}

/// A single assignment step, e.g. `s0 = a + (a << 3)`.
#[pyclass]
#[derive(Clone)]
struct Step {
    #[pyo3(get)]
    name: String,
    #[pyo3(get)]
    expr: String,
}

#[pymethods]
impl Step {
    fn __str__(&self) -> String {
        format!("{} = {}", self.name, self.expr)
    }
}

/// An ordered sequence of Steps that together compute `constant * input`.
#[pyclass]
#[derive(Clone)]
struct StepSequence {
    #[pyo3(get)]
    steps: Vec<Step>,
}

#[pymethods]
impl StepSequence {
    #[getter]
    fn result(&self) -> String {
        self.steps.last().map(|s| s.name.clone()).unwrap_or_else(|| "x".to_string())
    }

    fn __str__(&self) -> String {
        self.steps
            .iter()
            .map(|s| format!("{} = {}", s.name, s.expr))
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn __iter__(slf: PyRef<'_, Self>) -> PyResult<Py<PyAny>> {
        let py = slf.py();
        let list = PyList::empty(py);
        for s in &slf.steps {
            list.append(s.clone())?;
        }
        Ok(list.as_any().call_method0("__iter__")?.into())
    }
}

// Internal tree representation
#[derive(Clone)]
enum Tree {
    Leaf(usize), // constant (power-of-2 or 1)
    Node {
        variant: &'static str,
        left: Box<Tree>,
        right: Box<Tree>,
        p0: usize, // original p0 param (may encode a left-shift)
        p1: usize, // original p1 param (may encode a right-shift)
    },
    Leapfrog {
        variant: &'static str,
        params: Vec<usize>,
    },
}

// Expression helpers
fn is_base(c: usize) -> bool {
    c == 0 || c.count_ones() == 1
}

fn shift_expr(value: usize, inner: &str) -> String {
    if value == 1 {
        return inner.to_string();
    }
    let shift = value.trailing_zeros() as usize;
    if (1usize << shift) == value {
        format!("({inner} << {shift})")
    } else {
        format!("({value} * {inner})")
    }
}

fn leaf_expr(constant: usize, input_var: &str) -> String {
    shift_expr(constant, input_var)
}

fn needs_parens(expr: &str) -> bool {
    expr.contains(" + ") || expr.contains(" - ")
}

fn wrap(expr: &str) -> String {
    if needs_parens(expr) && !(expr.starts_with('(') && expr.ends_with(')')) {
        format!("({expr})")
    } else {
        expr.to_string()
    }
}

// Generate trees for a constant, using cache to avoid redundant work.
// Each tree corresponds to one way to compute the constant via the graph types.
fn build_trees(
    all_types: &[Vec<GraphType>],
    constant: usize,
    cache: &mut HashMap<usize, Vec<Tree>>,
) -> Vec<Tree> {
    if let Some(cached) = cache.get(&constant) {
        return cached.clone();
    }

    if is_base(constant) {
        let result = vec![Tree::Leaf(constant)];
        cache.insert(constant, result.clone());
        return result;
    }

    // look up graph types for this constant
    let position = constant / 2; // odd index → position
    let graph_types: Vec<GraphType> = if position < all_types.len() {
        all_types[position].clone()
    } else {
        vec![]
    };

    if graph_types.is_empty() {
        let result = vec![Tree::Leaf(constant)];
        cache.insert(constant, result.clone());
        return result;
    }

    let mut trees: Vec<Tree> = Vec::new();
    for gt in &graph_types {
        match gt.variant {
            "Adder" | "Subtractor" | "Cascade" => {
                let p0 = gt.params[0];
                let p1 = gt.params[1];

                // Strip power-of-2 factor from both params before recursion
                let p0_odd = if p0 > 0 { p0 >> p0.trailing_zeros() } else { p0 };
                let p1_odd = if p1 > 0 { p1 >> p1.trailing_zeros() } else { p1 };

                let left_trees = build_trees(all_types, p0_odd, cache);
                let right_trees = build_trees(all_types, p1_odd, cache);

                for lt in &left_trees {
                    for rt in &right_trees {
                        trees.push(Tree::Node {
                            variant: gt.variant,
                            left: Box::new(lt.clone()),
                            right: Box::new(rt.clone()),
                            p0,
                            p1,
                        });
                    }
                }

                // Two computation trees for Cascade (due to symmetry)
                if gt.variant == "Cascade" && p0 != p1 {
                    for lt in &right_trees {
                        for rt in &left_trees {
                            trees.push(Tree::Node {
                                variant: gt.variant,
                                left: Box::new(lt.clone()),
                                right: Box::new(rt.clone()),
                                p0: p1,
                                p1: p0,
                            });
                        }
                    }
                }
            }
            v => {
                trees.push(Tree::Leapfrog {
                    variant: v,
                    params: gt.params.clone(),
                });
            }
        }
    }

    cache.insert(constant, trees.clone());
    trees
}

/// Replace whole-word occurrences of `from` with `to` in `s`.
fn replace_word(s: &str, from: &str, to: &str) -> String {
    replace_words(s, |w| if w == from { Some(to) } else { None })
}

/// Single-pass word-boundary replacement driven by a lookup closure.
fn replace_words<'a, F>(s: &str, lookup: F) -> String
where
    F: Fn(&str) -> Option<&'a str>,
{
    let mut result = String::with_capacity(s.len());
    let chars: Vec<char> = s.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        if chars[i].is_alphanumeric() || chars[i] == '_' {
            // collect the full word token
            let start = i;
            while i < chars.len() && (chars[i].is_alphanumeric() || chars[i] == '_') {
                i += 1;
            }
            let word: String = chars[start..i].iter().collect();
            if let Some(replacement) = lookup(&word) {
                result.push_str(replacement);
            } else {
                result.push_str(&word);
            }
        } else {
            result.push(chars[i]);
            i += 1;
        }
    }
    result
}

/// Build an inline shift expression without emitting a step.
fn inline_shift(expr: &str, shift: i32) -> String {
    if shift == 0 {
        return expr.to_string();
    }
    if shift > 0 {
        format!("({expr} << {shift})")
    } else {
        let rshift = (-shift) as u32;
        format!("({expr} >> {rshift})")
    }
}

/// Emit a shift as a named step (use only when a step name is required as a return value).
fn emit_shift_scaled(expr: &str, shift: i32, steps: &mut Vec<Step>) -> String {
    if shift == 0 {
        return expr.to_string();
    }
    let name = format!("s{}", steps.len());
    steps.push(Step { name: name.clone(), expr: inline_shift(expr, shift) });
    name
}

/// Bit-length of a constant: smallest k such that 2^k > constant.
/// Used as the denominator exponent in scaled mode so constant/2^k < 1.
fn bit_length(c: usize) -> i32 {
    if c == 0 { 0 } else { (usize::BITS - c.leading_zeros()) as i32 }
}

/// Effective output scale of a sub-tree in scaled mode.
fn eff_scale(constant: usize, scaled: bool) -> i32 {
    if scaled && constant > 1 { bit_length(constant) } else { 0 }
}

// Flatten a Tree into a sequence of Steps.
fn flatten(
    tree: &Tree,
    node_scale: i32,
    steps: &mut Vec<Step>,
    input_var: &str,
) -> String {
    match tree {
        Tree::Leaf(c) => {
            if *c == 1 {
                return input_var.to_string();
            }
            let shift_raw = c.trailing_zeros() as i32;
            let net = shift_raw - node_scale;
            inline_shift(input_var, net)
        }

        Tree::Leapfrog { variant, params } => {
            let name = format!("s{}", steps.len());
            let expr = format!(
                "<{}({})>",
                variant,
                params.iter().map(|p| p.to_string()).collect::<Vec<_>>().join(", ")
            );
            steps.push(Step { name: name.clone(), expr });
            name
        }

        Tree::Node { variant, left, right, p0, p1 } => {
            let p0_shift_raw = if *p0 > 0 { p0.trailing_zeros() as i32 } else { 0 };
            let p1_shift_raw = if *p1 > 0 { p1.trailing_zeros() as i32 } else { 0 };
            let p0_odd = if *p0 > 0 { p0 >> p0.trailing_zeros() } else { *p0 };
            let p1_odd = if *p1 > 0 { p1 >> p1.trailing_zeros() } else { *p1 };
            // child scale: bit_length(child_odd) when scaled, else 0
            let scaled = node_scale > 0;
            let p0_child_scale = eff_scale(p0_odd, scaled);

            if *variant == "Cascade" {
                // Cascade computes  p1 * (p0 * x).
                let inner_scale = eff_scale(*p0, scaled);
                let outer_scale = node_scale - inner_scale;

                // Inner stage: flatten left sub-tree then shift down to denominator 2^inner_scale.
                let left_base = flatten(left, p0_child_scale, steps, input_var);
                let inner_net = p0_child_scale + p0_shift_raw - inner_scale;
                let intermediate = inline_shift(&left_base, inner_net);

                // Outer stage: flatten right sub-tree (placeholder input_var),
                // then replay substituting input_var → intermediate.
                let mut tmp_steps: Vec<Step> = Vec::new();
                let right_result = flatten(right, outer_scale, &mut tmp_steps, input_var);

                let wrapped = wrap(&intermediate);
                let mut name_remap: HashMap<String, String> = HashMap::new();
                for s in &tmp_steps {
                    let new_name = format!("s{}", steps.len());
                    let new_expr = replace_words(&s.expr, |w| {
                        if let Some(mapped) = name_remap.get(w) {
                            Some(mapped.as_str())
                        } else if w == input_var {
                            Some(wrapped.as_str())
                        } else {
                            None
                        }
                    });
                    name_remap.insert(s.name.clone(), new_name.clone());
                    steps.push(Step { name: new_name, expr: new_expr });
                }

                let remapped = if let Some(mapped) = name_remap.get(&right_result) {
                    mapped.clone()
                } else {
                    let subbed = replace_words(&right_result, |w| {
                        if w == input_var { Some(wrapped.as_str()) } else { None }
                    });
                    let new_name = format!("s{}", steps.len());
                    steps.push(Step { name: new_name.clone(), expr: subbed });
                    new_name
                };

                // Any trailing-zero shift in p1 on top of what the outer sub-tree handled.
                return emit_shift_scaled(&remapped, p1_shift_raw, steps);
            }

            let p0_child_scale_add = eff_scale(p0_odd, scaled).min((node_scale - p0_shift_raw).max(0));
            let p1_child_scale_add = eff_scale(p1_odd, scaled).min((node_scale - p1_shift_raw).max(0));

            let left_base  = flatten(left,  p0_child_scale_add, steps, input_var);
            let right_base = flatten(right, p1_child_scale_add, steps, input_var);

            let left_net  = p0_child_scale_add + p0_shift_raw - node_scale;
            let right_net = p1_child_scale_add + p1_shift_raw - node_scale;

            // If both operands need right-shifting, factor the common shift out to
            // the result.  E.g. instead of:
            //   s0 = x>>1;  s1 = x>>2;  s2 = s0 - s1
            // emit:
            //   s0 = x>>1;  s1 = x - s0;  s2 = s1>>1
            // Done by calculating the common right-shift and then subtracting this from both nets.
            let (left_net_adj, right_net_adj, result_shift) = if left_net < 0 && right_net < 0 {
                let factor = left_net.max(right_net); // least amount of right-shift (closest to 0)
                (left_net - factor, right_net - factor, factor)
            } else {
                (left_net, right_net, 0)
            };

            // For addition, prefer the shift on the right operand (cosmetic, since + is commutative).
            let (left_base_f, left_net_f, right_base_f, right_net_f) =
                if *variant == "Adder" && left_net_adj != 0 && right_net_adj == 0 {
                    (right_base.as_str(), right_net_adj, left_base.as_str(), left_net_adj)
                } else {
                    (left_base.as_str(), left_net_adj, right_base.as_str(), right_net_adj)
                };

            let left_expr  = inline_shift(left_base_f,  left_net_f);
            let right_expr = inline_shift(right_base_f, right_net_f);

            let add_sub_expr = match *variant {
                "Adder"      => format!("({} + {})", wrap(&left_expr), wrap(&right_expr)),
                "Subtractor" => format!("({} - {})", wrap(&left_expr), wrap(&right_expr)),
                other        => format!("<{other}>"),
            };

            // Fold the result shift directly into this step's expression.
            let expr = if result_shift == 0 {
                add_sub_expr
            } else {
                inline_shift(&add_sub_expr, result_shift)
            };

            let name = format!("s{}", steps.len());
            steps.push(Step { name: name.clone(), expr });
            name
        }
    }
}


/// Return all equation sequences for `constant * input_var`.
/// Each StepSequence holds a list of Step objects; str(seq) prints them.
/// When `scaled=True`, equations are safe-scaled to not overflow.
#[pyfunction]
#[pyo3(signature = (constant, input_var = "x", output_var = None, scaled = false))]
fn get_equations(
    py: Python,
    constant: usize,
    input_var: &str,
    output_var: Option<&str>,
    scaled: bool,
) -> PyResult<Py<PyList>> {
    let all_types = get_graph_types_data()?;

    // The graph-type data is indexed by odd numbers only.
    let odd_constant = if constant > 0 {
        constant >> constant.trailing_zeros()
    } else {
        constant
    };

    // Build every possible computation tree for `odd_constant * input_var`.
    let mut cache: HashMap<usize, Vec<Tree>> = HashMap::new();
    let trees = build_trees(&all_types, odd_constant, &mut cache);

    // Convert each tree into a flat list of named assignment steps, then
    // wrap the list in a StepSequence and append it to the Python result list.
    let result = PyList::empty(py);
    for tree in &trees {
        let mut steps: Vec<Step> = Vec::new();

        // if *scaled* is true, ensure values are kept in the range [0, 1)
        // by treating them as fixed-point numbers with `node_scale` fractional bits.
        let node_scale = if scaled { bit_length(odd_constant) } else { 0 };

        // Walk the tree and emit steps; `last_name` is the variable that holds
        // the final result after all steps have been emitted.
        let last_name = flatten(tree, node_scale, &mut steps, input_var);

        if steps.is_empty() {
            // Trivial case: the constant is 1, 0, or a pure power-of-two,
            // so a single direct assignment is all that is needed.
            let name = output_var.unwrap_or("s0").to_string();
            steps.push(Step { name, expr: leaf_expr(odd_constant, input_var) });
        } else if let Some(out) = output_var {
            // Rename the result variable to the requested output name.
            // 1. Rename the last step itself (it is the one that produces the result).
            // 2. Update any prior steps that happen to reference `last_name`
            //    in their expression (e.g. after Cascade substitution).
            if let Some(last) = steps.last_mut() {
                last.name = out.to_string();
            }
            for s in steps.iter_mut().rev().skip(1) {
                s.expr = replace_word(&s.expr, &last_name, out);
            }
        }

        result.append(StepSequence { steps })?;
    }

    Ok(result.into())
}

/// Pretty-print all equation sequences for a constant.
#[pyfunction]
#[pyo3(signature = (constant, input_var = "x", output_var = "y", scaled = false))]
fn print_equations(
    py: Python,
    constant: usize,
    input_var: &str,
    output_var: Option<&str>,
    scaled: bool,
) -> PyResult<()> {
    let seqs: Vec<StepSequence> = get_equations(py, constant, input_var, output_var, scaled)?
        .bind(py)
        .iter()
        .map(|item| item.extract::<StepSequence>().map_err(PyErr::from))
        .collect::<PyResult<_>>()?;

    println!("# {}*{}  —  {} combination(s)\n", constant, input_var, seqs.len());
    for (i, seq) in seqs.iter().enumerate() {
        println!("# combination {}  (result: {})", i, seq.result());
        println!("{}", seq.__str__());
        println!();
    }
    Ok(())
}

#[pymodule]
fn constant_multiplication(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<GraphType>()?;
    m.add_class::<Step>()?;
    m.add_class::<StepSequence>()?;
    m.add_function(wrap_pyfunction!(adder_cost, m)?)?;
    m.add_function(wrap_pyfunction!(info, m)?)?;
    m.add_function(wrap_pyfunction!(get_graph_types, m)?)?;
    m.add_function(wrap_pyfunction!(get_all_graph_types, m)?)?;
    m.add_function(wrap_pyfunction!(get_equations, m)?)?;
    m.add_function(wrap_pyfunction!(print_equations, m)?)?;
    Ok(())
}
