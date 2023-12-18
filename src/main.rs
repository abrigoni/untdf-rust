use ayp_1::{tp1::{ej8_a, ej8_b, ej8_d, ej8_e, ej8_f, ej8_g, ej8_h}, tp2::{ej5, ej6}};


mod ayp_1 {
    pub mod tp1 {
        pub mod ej8_a;
        pub mod ej8_b;
        pub mod ej8_d;
        pub mod ej8_e;
        pub mod ej8_f;
        pub mod ej8_g;
        pub mod ej8_h;
    }

    pub mod tp2 {
        pub mod ej5;
        pub mod ej6;
    }
}


fn main() {
    // TP 1
    ej8_a::tp1_ej8_a();
    ej8_b::tp1_ej8_b();
    ej8_d::tp1_ej8_d();
    ej8_e::tp1_ej8_e();
    ej8_f::tp1_ej8_f();
    ej8_g::tp1_ej8_g();
    ej8_h::tp1_ej8_h();

    // TP 2
    ej5::ej5();
    ej6::ej6();
}








