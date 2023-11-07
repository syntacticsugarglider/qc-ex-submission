use std::{collections::HashSet, process::Command};

use assert_cmd::prelude::*;

use assert_fs::prelude::*;

#[test]
fn integration() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("cookie_log.csv")?;

    file.write_str(
        r#"cookie,timestamp
AtY0laUfhglK3lC7,2018-12-09T14:19:00+00:00
SAZuXPGUrfbcn5UA,2018-12-09T10:13:00+00:00
5UAVanZf6UtGyKVS,2018-12-09T07:25:00+00:00
AtY0laUfhglK3lC7,2018-12-09T06:19:00+00:00
SAZuXPGUrfbcn5UA,2018-12-08T22:03:00+00:00
4sMM2LxV07bPJzwf,2018-12-08T21:30:00+00:00
fbcn5UAVanZf6UtG,2018-12-08T09:30:00+00:00
4sMM2LxV07bPJzwf,2018-12-07T23:30:00+00:00"#,
    )?;

    let mut cmd = Command::cargo_bin("most_active_cookie")?;
    cmd.arg("-d").arg("2018-12-08").arg(file.path());
    cmd.assert()
        .success()
        .stdout(predicates::function::function(|output: &str| {
            output
                .trim()
                .split('\n')
                .map(str::to_owned)
                .collect::<HashSet<String>>()
                == vec!["SAZuXPGUrfbcn5UA", "4sMM2LxV07bPJzwf", "fbcn5UAVanZf6UtG"]
                    .into_iter()
                    .map(str::to_owned)
                    .collect()
        }));

    Ok(())
}
