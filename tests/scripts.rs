use std::fs;
use std::io::{self, Read};
use std::process::{Command, Stdio};

#[test]
fn test_scripts() -> anyhow::Result<()> {
    fs::create_dir_all("temp")?;

    for entry in fs::read_dir("test-scripts")? {
        let entry = entry?;
        let path = entry.path();

        let extension = path.extension().and_then(|ext| ext.to_str());
        if extension != Some("lua") {
            continue;
        }

        let mut child = Command::new(env!("CARGO_BIN_EXE_remodel"))
            .arg("run")
            .arg(&path)
            .args(&["arg1", "arg2", "arg3"])
            .stdout(Stdio::piped())
            .spawn()?;

        let mut stdout = child.stdout.take().unwrap();
        let status = child.wait()?;

        assert!(
            status.success(),
            "Test {} failed with an error",
            path.display()
        );

        let mut actual = String::new();
        stdout.read_to_string(&mut actual)?;

        let expected_stdout_path = path.with_extension("expected");
        match fs::read_to_string(&expected_stdout_path) {
            Ok(expected) => {
                let expected = expected.trim().replace("\r\n", "\n");
                let actual = actual.trim().replace("\r\n", "\n");

                if expected != actual {
                    panic!(
                        "Output from test {} did not match expected output.
Expected output:
{}

Actual output:
{}",
                        path.display(),
                        expected,
                        actual
                    );
                }
            }

            Err(err) => {
                if err.kind() != io::ErrorKind::NotFound {
                    panic!(
                        "Could not read expected output file from {}",
                        expected_stdout_path.display()
                    );
                }
            }
        }
    }

    Ok(())
}
