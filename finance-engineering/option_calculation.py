import sympy as sp
from dataclasses import dataclass
from typing import List, Set, Tuple, Literal
from pprint import pp


@dataclass
class Node:
    up_count: int
    down_count: int
    value: float

    def __hash__(self):
        return hash((self.up_count, self.down_count))


def sorted_tuple(t: Tuple):
    return tuple(sorted(t))


def calculate_risk_neutral_probability(r, dt, u, d):
    """
    Calculate risk-neutral probability (often denoted as ``p``).

    :param r: risk-free rate (e.g. ``0.1`` for 10%)
    :param dt: time step size in year (e.g. ``1`` for one year; ``0.5`` for 6 months)
    :param u: down factor (e.g. ``0.8`` for -20%)
    :param d: up factor (e.g. ``1.2`` for 20%)
    :return:
    """
    return (sp.exp(r * dt) - d) / (u - d)


def calculate_discount_factor(r, dt):
    """
    Calculate the discount factor.

    :param dt: time step size in year (e.g. ``1`` for one year; ``0.5`` for 6 months)
    :param r: risk-free rate (e.g. ``0.1`` for 10%)
    :return:
    """

    return sp.exp(-r * dt)


def calculate_next_nodes(prev_nodes: Set[Node], u, d):
    """
    Calculate the next nodes in a binomial tree.

    :param prev_nodes: a set of previous nodes
    :param u: up factor (e.g. ``1.2`` for 20%)
    :param d: down factor (e.g. ``0.8`` for -20%)
    :return:
    """
    next_nodes = set()

    for node_current in prev_nodes:
        node_up = Node(
            value=node_current.value * u,
            up_count=node_current.up_count + 1,
            down_count=node_current.down_count,
        )
        node_down = Node(
            value=node_current.value * d,
            up_count=node_current.up_count,
            down_count=node_current.down_count + 1,
        )
        next_nodes.add(node_up)
        next_nodes.add(node_down)

    return next_nodes


def generate_binomial_tree(S0, u, d, N):
    """
    Generate a binomial tree for a stock.

    :param S0: initial stock price
    :param u: up factor (e.g. ``1.2`` for 20%)
    :param d: down factor (e.g. ``0.8`` for -20%)
    :param N: number of steps
    :return:
    """

    initial_node = Node(value=S0, up_count=0, down_count=0)
    tree = [{initial_node}]

    for _ in range(N):
        next_nodes = calculate_next_nodes(prev_nodes=tree[-1], u=u, d=d)
        tree.append(next_nodes)

    return tree


def calculate_option_price(
    tree: List[Set[Node]],
    option_type: Literal["call", "put"],
    option_style: Literal["american", "european"],
    K,
    r,
    T,
    u,
    d,
):
    """
    Calculate option price by backward induction.

    :param tree: generated binomial tree
    :param option_type: either "call" or "put"
    :param option_style: either "american" or "european"
    :param K: strike price
    :param r: risk-free rate (e.g. 0.1 for 10%)
    :param T: time to expiration in years
    :param u: up factor (e.g. 1.2 for 20% increase)
    :param d: down factor (e.g. 0.8 for 20% decrease)
    :return: option price
    """

    N = len(tree) - 1
    dt = T / N
    p = calculate_risk_neutral_probability(r, dt, u, d)
    discount = calculate_discount_factor(r, dt)
    
    # Create a dictionary to store option values for each node
    # Key: (up_count, down_count), Value: option_value
    option_values = {}
    
    # Step 1: Calculate option values at terminal nodes (time T)
    for node in tree[-1]:
        key = (node.up_count, node.down_count)
        if option_type == "call":
            option_values[key] = max(node.value - K, 0)
        else:  # put
            option_values[key] = max(K - node.value, 0)

    # Step 2: Work backwards through the tree
    # Start from N-1 and go to 0
    for t in reversed(range(N)):
        for node in tree[t]:
            # Generate the keys for the two successor nodes
            key = (node.up_count, node.down_count)
            up_key = (node.up_count + 1, node.down_count)
            down_key = (node.up_count, node.down_count + 1)

            continuation_value = discount * (
                p * option_values[up_key] + (1 - p) * option_values[down_key]
            )
            match option_style, option_type:
                case ("european", _):
                    option_values[key] = continuation_value
                case ("american", "call"):
                    intrinsic_value = max(node.value - K, 0)
                    option_values[key] = max(continuation_value, intrinsic_value)
                case ("american", "put"):
                    intrinsic_value = max(0, K - node.value)
                    option_values[key] = max(continuation_value, intrinsic_value)
                case _:
                    raise ValueError("unreachable code; (option_style, option_type): " + str((option_style, option_type)))

    return option_values[(0, 0)]


def main():
    S0, u, d, N = 100, 1.2, 0.8, 10
    K, r, T = 90, 0.1, 10
    option_type: Literal["put", "call"] = "call"
    option_style: Literal["american", "european"] = "european"
    tree = generate_binomial_tree(
        S0=S0,
        u=u,
        d=d,
        N=N,
    )
    option_price = calculate_option_price(
        tree=tree,
        option_type=option_type,
        option_style=option_style,
        K=K,
        r=r,
        T=T,
        u=u,
        d=d,
    )
    pp(option_price)


if __name__ == "__main__":
    main()