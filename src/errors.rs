use std::io;
use std::path::PathBuf;

error_chain! {
    foreign_links {
        IOError(io::Error);
    }
    errors {
        UnknownInputFile(path: PathBuf) {
            description("Unable to use provided file")
                display("Unable to use provided file: '{}'", path.display())
        }
        UnrecognizedArchive(path: PathBuf) {
            description("Unrecognized Archive Type")
                display("Could not determine type of archive: {}", path.display())
        }
        ExecError(t: String) {
            description("Error calling into exec")
                display("Failure to exec: {}", t)
        }
    }
}
