use iced::widget::{button, column, container, row, scrollable, text};
use iced::{Element, Length, Task};
use tokio::process::Command;

#[derive(Default)]
struct BluetoothApp {
    terminal_output: String,
}

#[derive(Debug, Clone)]
enum Message {
    RunCommand(String),
    CommandFinished(String),
}

fn update(state: &mut BluetoothApp, message: Message) -> Task<Message> {
    match message {
        Message::RunCommand(cmd) => {
            state.terminal_output = format!("Running '{}'...", cmd);

            Task::perform(run_command_async(cmd), Message::CommandFinished)
        }
        Message::CommandFinished(result) => {
            state.terminal_output = result;
            Task::none()
        }
    }
}

async fn run_command_async(cmd: String) -> String {
    let cmds: Vec<&str> = cmd.split_whitespace().collect();
    let output = Command::new("bluetoothctl").args(cmds).output().await;

    match output {
        Ok(out) => {
            let stdout = String::from_utf8_lossy(&out.stdout).to_string();
            let stderr = String::from_utf8_lossy(&out.stderr).to_string();

            if out.status.success() {
                if stdout.trim().is_empty() {
                    format!("Success: {}\n{}", cmd, stderr)
                } else {
                    stdout
                }
            } else {
                format!("Error running '{}':\n{}", cmd, stderr)
            }
        }
        Err(e) => format!("Failed to execute process: {}", e),
    }
}

fn view(state: &BluetoothApp) -> Element<'_, Message> {
    let controls = row![
        button("Power On").on_press(Message::RunCommand("power on".to_string())),
        button("Power Off").on_press(Message::RunCommand("power off".to_string())),
        button("Devices").on_press(Message::RunCommand("devices".to_string())),
    ]
    .spacing(15);

    let lines = state
        .terminal_output
        .lines()
        .map(|line| button(line).into());

    let content = column![
        text("Bluetooth Controller").size(32),
        controls,
        text("Terminal Output:").size(20),
        /* container(scrollable(text(&state.terminal_output)))
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(10)
        .style(|theme: &iced::Theme| {
            let palette = theme.extended_palette();
            container::Style::default().background(palette.background.weak.color)
        }),
        */
        container(scrollable(column(lines).spacing(10).padding(10)))
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(10)
            .style(|theme: &iced::Theme| {
                let palette = theme.extended_palette();
                container::Style::default().background(palette.background.weak.color)
            }),
    ]
    .spacing(20)
    .padding(30);

    content.into()
}

fn main() -> iced::Result {
    iced::application(BluetoothApp::default, update, view)
        .window_size((600.0, 450.0))
        .centered()
        .run()
}
