/*
This example is a simple iOS-style calculator. This particular example can run any platform - Web, Mobile, Desktop.
This calculator version uses React-style state management. All state is held as individual use_states.
*/

use dioxus::events::*;
use dioxus::prelude::*;
use separator::Separatable;

fn main() {
    use dioxus::desktop::tao::dpi::LogicalSize;
    dioxus::desktop::launch_cfg(app, |cfg| {
        cfg.with_window(|w| {
            w.with_title("Calculator Demo")
                .with_resizable(false)
                .with_inner_size(LogicalSize::new(320.0, 530.0))
        })
    });
}

fn app(cx: Scope) -> Element {
    let cur_val = use_state(&cx, || 0.0_f64);
    let operator = use_state(&cx, Option::default);
    let display_value = use_state(&cx, String::new);

    let input_digit = move |num: u8| display_value.modify().push_str(num.to_string().as_str());

    cx.render(rsx!(
        style { [include_str!("./assets/calculator.css")] }
        div { id: "wrapper",
            div { class: "app",
                div { class: "calculator",
                    onkeydown: move |evt| match evt.key_code {
                        KeyCode::Add => operator.set(Some("+")),
                        KeyCode::Subtract => operator.set(Some("-")),
                        KeyCode::Divide => operator.set(Some("/")),
                        KeyCode::Multiply => operator.set(Some("*")),
                        KeyCode::Num0 => input_digit(0),
                        KeyCode::Num1 => input_digit(1),
                        KeyCode::Num2 => input_digit(2),
                        KeyCode::Num3 => input_digit(3),
                        KeyCode::Num4 => input_digit(4),
                        KeyCode::Num5 => input_digit(5),
                        KeyCode::Num6 => input_digit(6),
                        KeyCode::Num7 => input_digit(7),
                        KeyCode::Num8 => input_digit(8),
                        KeyCode::Num9 => input_digit(9),
                        KeyCode::Backspace => {
                            if !display_value.as_str().eq("0") {
                                display_value.modify().pop();
                            }
                        }
                        _ => {}
                    },
                    div { class: "calculator-display", [cur_val.separated_string()] }
                    div { class: "calculator-keypad",
                        div { class: "input-keys",
                            div { class: "function-keys",
                                button {
                                    class: "calculator-key key-clear",
                                    onclick: move |_| {
                                        display_value.set("0".to_string());
                                        if display_value != "0" {
                                            operator.set(None);
                                            cur_val.set(0.0);
                                        }
                                    },
                                    [if display_value == "0" { "C" } else { "AC" }]
                                }
                                button {
                                    class: "calculator-key key-sign",
                                    onclick: move |_| {
                                        if display_value.starts_with("-") {
                                            display_value.set(display_value.trim_start_matches("-").to_string())
                                        } else {
                                            display_value.set(format!("-{}", *display_value))
                                        }
                                    },
                                    "±"
                                }
                                button {
                                    class: "calculator-key key-percent",
                                    onclick: move |_| {
                                        let pct = (display_value.parse::<f64>().unwrap() / 100.0).to_string();
                                        display_value.set(pct);
                                    },
                                    "%"
                                }
                            }
                            div { class: "digit-keys",
                                button { class: "calculator-key key-0", onclick: move |_| input_digit(0),
                                    "0"
                                }
                                button { class: "calculator-key key-dot", onclick: move |_| display_value.modify().push_str("."),
                                    "●"
                                }
                                (1..10).map(|k| rsx!{
                                    button {
                                        class: "calculator-key {k}",
                                        name: "key-{k}",
                                        onclick: move |_| input_digit(k),
                                        "{k}"
                                    }
                                }),
                            }
                        }
                        div { class: "operator-keys",
                            button { class: "calculator-key key-divide",
                                onclick: move |_| operator.set(Some("/")),
                                "÷"
                            }
                            button { class: "calculator-key key-multiply",
                                onclick: move |_| operator.set(Some("*")),
                                "×"
                            }
                            button { class: "calculator-key key-subtract",
                                onclick: move |_| operator.set(Some("-")),
                                "−"
                            }
                            button { class: "calculator-key key-add",
                                onclick: move |_| operator.set(Some("+")),
                                "+"
                            }
                            button { class: "calculator-key key-equals",
                                onclick: move |_| {
                                    if let Some(op) = operator.as_ref() {
                                        let rhs = display_value.parse::<f64>().unwrap();
                                        let new_val = match *op {
                                            "+" => *cur_val + rhs,
                                            "-" => *cur_val - rhs,
                                            "*" => *cur_val * rhs,
                                            "/" => *cur_val / rhs,
                                            _ => unreachable!(),
                                        };
                                        cur_val.set(new_val);
                                        display_value.set(new_val.to_string());
                                        operator.set(None);
                                    }
                                },
                                "="
                            }
                        }
                    }
                }
            }
        }

    ))
}
