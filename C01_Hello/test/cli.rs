/*
--version 1
#[test]
fn works(){
    assert!(true);
}
*/
/*
-- version 2
use std::process::Command;
#[test]
fn runs(){
    let mut cmd = Command::new("ls");
    let res = cmd.output();
    asset!(res.is_ok());
}
 */
use assert_cmd::Command;

#[test]
fn runs(){
    //在这里的test case 是如何和主程序关联上的，不清楚，我在修改了cargo_bin参数之后，cargo test 还是
    //运行的main.rs 有点不能理解
    let mut cmd = Command::cargo_bin("c01_hello").unwarp();
    cmd.assert().success().stdout("hello, world!\n");
}

#[test]
fn true_ok(){
    let mut cmd = Command::cargo_bin("true").unwarp();
    cmd.assert().success();
}

#[test]
fn false_not_ok(){
    let mut cmd = Command::cargo_bin("false").unwarp();
    cmd.assert().failure();
}