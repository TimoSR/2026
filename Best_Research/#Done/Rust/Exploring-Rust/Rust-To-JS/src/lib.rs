// src/lib.rs

#[macro_export]
macro_rules! create_button {
    ($elem_id:expr, $initial_text:expr, $color:expr) => {
        println!(
            "
            document.addEventListener('DOMContentLoaded', (event) => {{
                const btn = document.createElement('button');
                btn.id = '{}';
                btn.innerText = '{}';
                btn.style.backgroundColor = '{}';
                document.body.appendChild(btn);

                let count = 0;
                btn.addEventListener('click', () => {{
                    count += 1;
                    btn.innerText = 'Clicked ' + count + ' times';
                }});
            }});
            ",
            $elem_id, $initial_text, $color
        );
    };
}
