# dioxus study


## toolchain
1. 安装rust，rust官网
```sh
# https://www.rust-lang.org/zh-CN/tools/install
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
2. 安装dioxus
```sh
cargo install cargo-binstall # 这个编译了很久
cargo binstall dioxus-cli

3. 准备tailwindcss环境
```sh
cd ui
npx tailwindcss init
npm install

# start tailwindcss
# 准备一个input文件, 然后启动tailwind
npx tailwindcss -i ./input.css -o ./assets/tailwind.css --watch
```


4. 添加Dioxus.tomal启动dioxus
```sh
dx serve
```

## tailwind style website
```rust
https://flowbite.com/docs/components/
https://tailwindflex.com/    //官网的包不需要额外的安装
https://daisyui.com/components/mockup-code/
```
-------------------------------------------
Rules of Hooks
The above example might seem a bit magic, since Rust functions are typically not associated with state. Dioxus allows hooks to maintain state across renders through a reference to ScopeState, which is why you must pass &cx to them.

But how can Dioxus differentiate between multiple hooks in the same component? As you saw in the second example, both use_state functions were called with the same parameters, so how come they can return different things when the counters are different?

src/hooks_counter_two_state.rs

let mut count_a = use_state(cx, || 0);
let mut count_b = use_state(cx, || 0);
This is only possible because the two hooks are always called in the same order, so Dioxus knows which is which. Because the order you call hooks matters, you must follow certain rules when using hooks:

Hooks may be only used in components or other hooks (we'll get to that later)
On every call to the component function
The same hooks must be called
In the same order
Hooks name's should start with use_ so you don't accidentally confuse them with regular functions
These rules mean that there are certain things you can't do with hooks:

No Hooks in Conditionals
src/hooks_bad.rs

// ❌ don't call hooks in conditionals!
// We must ensure that the same hooks will be called every time
// But `if` statements only run if the conditional is true!
// So we might violate rule 2.
if you_are_happy && you_know_it {
let something = use_state(cx, || "hands");
println!("clap your {something}")
}

// ✅ instead, *always* call use_state
// You can put other stuff in the conditional though
let something = use_state(cx, || "hands");
if you_are_happy && you_know_it {
println!("clap your {something}")
}
No Hooks in Closures
src/hooks_bad.rs

// ❌ don't call hooks inside closures!
// We can't guarantee that the closure, if used, will be called in the same order every time
let _a = || {
let b = use_state(cx, || 0);
b.get()
};

// ✅ instead, move hook `b` outside
let b = use_state(cx, || 0);
let _a = || b.get();
No Hooks in Loops
src/hooks_bad.rs

// `names` is a Vec<&str>

// ❌ Do not use hooks in loops!
// In this case, if the length of the Vec changes, we break rule 2
for _name in &names {
let is_selected = use_state(cx, || false);
println!("selected: {is_selected}");
}

// ✅ Instead, use a hashmap with use_ref
let selection_map = use_ref(cx, HashMap::<&str, bool>::new);

for name in &names {
let is_selected = selection_map.read()[name];
println!("selected: {is_selected}");
}
use_ref Hook
use_state is great for tracking simple values. However, you may notice in the UseState API that the only way to modify its value is to replace it with something else (e.g., by calling set, or through one of the +=, -= operators). This works well when it is cheap to construct a value (such as any primitive). But what if you want to maintain more complex data in the components state?

For example, suppose we want to maintain a Vec of values. If we stored it with use_state, the only way to add a new value to the list would be to create a new Vec with the additional value, and put it in the state. This is expensive! We want to modify the existing Vec instead.

Thankfully, there is another hook for that, use_ref! It is similar to use_state, but it lets you get a mutable reference to the contained data.

Here's a simple example that keeps a list of events in a use_ref. We can acquire write access to the state with .with_mut(), and then just .push a new value to the state:

src/hooks_use_ref.rs

fn App(cx: Scope) -> Element {
    let list = use_ref(cx, Vec::new);

    cx.render(rsx!(
        p { "Current list: {list.read():?}" }
        button {
            onclick: move |event| {
                list.with_mut(|list| list.push(event));
            },
            "Click me!"
        }
    ))
}
The return values of use_state and use_ref ( UseState and UseRef, respectively) are in some ways similar to Cell and RefCell – they provide interior mutability. However, these Dioxus wrappers also ensure that the component gets re-rendered whenever you change the state.
