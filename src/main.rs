mod astar;
mod astar_test;
mod method_1;
mod rand_test;
mod tester;

fn main() {
    method_1::solve(2, astar::find);
    //astar_test::run();
    //tester::run(astar::find);
    rand_test::run();
}
