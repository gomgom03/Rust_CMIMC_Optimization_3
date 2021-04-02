mod astar;
mod astar_test;
mod method_1;
mod tester;
mod rand_test;

fn main() {
    method_1::solve(1, astar::find);
    //astar_test::run();
    //tester::run(astar::find);
    rand_test::run();
}
