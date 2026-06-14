#[derive(Debug)]
struct ExportReport {
    file_name: String,
    row_count: usize,
}

#[derive(Debug)]
enum ExportError {
    StorageUnavailable,
}

#[derive(Debug)]
enum TaskState<T, E> {
    Pending,
    Running { progress_percent: u8 },
    Completed(T),
    Failed(E),
}

fn render_status<T: std::fmt::Debug, E: std::fmt::Debug>(state: &TaskState<T, E>) -> String {
    match state {
        TaskState::Pending => "waiting for a worker".to_string(),
        TaskState::Running { progress_percent } => format!("running: {progress_percent}%"),
        TaskState::Completed(value) => format!("completed: {value:?}"),
        TaskState::Failed(error) => format!("failed: {error:?}"),
    }
}

fn main() {
    let states: [TaskState<ExportReport, ExportError>; 4] = [
        TaskState::Pending,
        TaskState::Running {
            progress_percent: 60,
        },
        TaskState::Completed(ExportReport {
            file_name: "orders.csv".to_string(),
            row_count: 1_024,
        }),
        TaskState::Failed(ExportError::StorageUnavailable),
    ];

    for state in &states {
        println!("{}", render_status(state));
    }

    if let TaskState::Completed(report) = &states[2] {
        println!("download {} with {} rows", report.file_name, report.row_count);
    }
}
