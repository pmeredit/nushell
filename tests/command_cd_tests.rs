mod helpers;

#[test]
fn cd_directory_not_found() {
    let actual = nu_error!(
    	cwd: "tests/fixtures",
    	"cd dir_that_does_not_exist"
    );

    assert!(actual.contains("dir_that_does_not_exist"));
    assert!(actual.contains("directory not found"));
}
