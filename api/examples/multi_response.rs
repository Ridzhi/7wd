use std::fmt::Display;

struct Resp {
    content: String,
}

trait IntoResp{
    fn into_resp(self) -> Resp;
}

impl IntoResp for u8 {
    fn into_resp(self) -> Resp {
        Resp { content: self.to_string() }
    }
}

impl IntoResp for u32 {
    fn into_resp(self) -> Resp {
        Resp { content: self.to_string() }
    }
}

impl IntoResp for Resp {
    fn into_resp(self) -> Resp {
        self
    }
}

fn main() {
    println!("Hello, world!");
    let r1 = handler_1();
    let r2 = handler_2();
    let r3 = handler_3();
    
    
    resolve(r1);
    resolve(r2);
    resolve(r3);
    
    // handler_1().into_code();
    // handler_2().into_code();
    // handler_3().into_code();
}

fn resolve(h: impl IntoResp) {
    println!("{}", h.into_resp().content);
}

fn handler_1() -> impl IntoResp {
    10u8
}

fn handler_2() -> impl IntoResp {
    10u32
}

fn handler_3() -> Resp {
    if true {
        return 10u8.into_resp();
    };

    10u32.into_resp()
}