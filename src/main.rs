mod astar;
mod astar_test;
mod method_1;
mod tester;

fn main() {
    method_1::solve(4, astar::find);
    //astar_test::run();
    //tester::run(astar::find);
}
