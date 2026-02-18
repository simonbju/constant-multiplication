import math

import constant_multiplication as cm
import pytest


def _check_constant(constant: int, scaled: bool) -> None:
    eq_trees = cm.get_equations(constant, input_var="x", output_var="y", scaled=scaled)
    assert eq_trees, f"No equation sequences returned for {constant}"

    for eq_tree in eq_trees:
        for step in eq_tree:
            print(f"{step.name} = {step.expr}")

        # Evaluate each step with x=1 and verify the final result equals constant
        ns: dict = {"x": 2**math.ceil(math.log2(constant))} if scaled else {"x": 1}
        for step in eq_tree:
            ns[step.name] = eval(step.expr, {"__builtins__": {}}, ns)
            print(f"{step.name} = {ns[step.name]}")

        result = ns["y"]
        print(f"  → result: {result}, expected: {constant}")
        assert result == constant, f"Expected {constant}, got {result}"


@pytest.mark.parametrize("scaled", [False, True])
def test_radix_3_equations(scaled: bool) -> None:
    # Two adds
    for constant in [143, 93, 75, 49]:
        _check_constant(constant, scaled=scaled)
    # Three adds
    for constant in [9159, 95229, 4815, 6303]:
        _check_constant(constant, scaled=scaled)
    # Four adds
    for constant in [4689375, 761835, 308169, 6454371]:
        _check_constant(constant, scaled=scaled)
