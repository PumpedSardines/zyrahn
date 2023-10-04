# Zyrahn

A cool programming language mainly made to learn more about programming and how compilers work.

## Example code:

```
fnc main() {
    var input_str: str = "3";
    var output_int: int = 0;

    if !std::str_to_int(input_str, output_int) {
        std::panic("Could not parse input_str")
    }

    var value: int = sum(output_int, 6) ** 2;

    std::print(
        std::to_str(field_a)
    );
}

fnc sum(a: int, b: int) -> in {
    ret a + b;
}
```
