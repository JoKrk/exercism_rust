#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {

    let mut stack = Vec::new();


    for input in inputs.iter() {

        match input {
            CalculatorInput::Add => {

                if stack.len() < 2 {
                    return None;
                }
                let y = stack.pop().unwrap_or_default();
                let x = stack.pop().unwrap_or_default();
                let new_val = x + y;
                println!("add operation, x: {}, y: {}, output {}", x, y, new_val);
                stack.push(new_val)    
            },
            CalculatorInput::Subtract => {

                if stack.len() < 2 {
                    return None;
                }
                let y = stack.pop().unwrap_or_default();
                let x = stack.pop().unwrap_or_default();
                let new_val = x - y;
                println!("subtract operation, x: {}, y: {}, output {}", x, y, new_val);
                stack.push(new_val)    
            },
            CalculatorInput::Multiply => {
                if stack.len() < 2 {
                    return None;
                }
                let y = stack.pop().unwrap_or_default();
                let x = stack.pop().unwrap_or_default();
                let new_val = x * y;
                println!("multiply operation, x: {}, y: {}, output {}", x, y, new_val);
                stack.push(new_val)   
            },
            CalculatorInput::Divide => {
                if stack.len() < 2 {
                    return None;
                }
                let y = stack.pop().unwrap_or_default();
                let x = stack.pop().unwrap_or_default();
                let new_val = x / y;
                println!("divide operation, x: {}, y: {}, output {}", x, y, new_val);
                stack.push(new_val)   
            },
            CalculatorInput::Value(x) => {
                stack.push(*x);
            },
        }

    }


    if stack.len() == 1 {
        stack.pop()
    } else {
        None
    }
}
