use std::path::{Path, PathBuf};

use chrono::NaiveDate;
use genpdfi::elements::{Paragraph, TableLayout, Text};
use genpdfi::fonts::{FontData, FontFamily};
use genpdfi::style::{self, Style};
use genpdfi::{Alignment, Document, Element, Margins, SimplePageDecorator};

use crate::error::AppError;
use crate::formatting::format_duration;
use crate::services::report::Report;

const FONT_REGULAR: &[u8] = include_bytes!("../../assets/fonts/LiberationSans-Regular.ttf");
const FONT_BOLD: &[u8] = include_bytes!("../../assets/fonts/LiberationSans-Bold.ttf");

fn load_font_data(data: &[u8]) -> Result<FontData, AppError> {
    FontData::new(data.to_vec(), None)
        .map_err(|e| AppError::SystemError(format!("Failed to load font: {e}")))
}

fn load_font_family() -> Result<FontFamily<FontData>, AppError> {
    Ok(FontFamily {
        regular: load_font_data(FONT_REGULAR)?,
        bold: load_font_data(FONT_BOLD)?,
        italic: load_font_data(FONT_REGULAR)?,
        bold_italic: load_font_data(FONT_BOLD)?,
    })
}

fn bold_string(s: &str) -> style::StyledString {
    style::StyledString::new(s.to_owned(), Style::new().bold(), None)
}

/// Render a report as a PDF file at the given output path.
pub fn render_pdf(report: &Report, output_path: &Path) -> Result<(), AppError> {
    let font_family = load_font_family()?;
    let mut doc = Document::new(font_family);
    doc.set_title("Time Report");

    let mut decorator = SimplePageDecorator::new();
    decorator.set_margins(Margins::vh(15, 15));
    doc.set_page_decorator(decorator);
    doc.set_font_size(10);

    // Header
    let mut title = Paragraph::new("");
    title.push(bold_string("Time Report"));
    title.set_alignment(Alignment::Center);
    doc.push(title);

    doc.push(Paragraph::new(format!(
        "{} to {}",
        report.from.format("%Y-%m-%d"),
        report.to.format("%Y-%m-%d")
    )));

    doc.push(Paragraph::new(format!(
        "Generated: {}",
        chrono::Local::now().format("%Y-%m-%d %H:%M")
    )));

    doc.push(genpdfi::elements::Break::new(1.5));

    // SECTION 1: Project Summary
    let mut summary_heading = Paragraph::new("");
    summary_heading.push(bold_string("Project Summary"));
    doc.push(summary_heading);
    doc.push(genpdfi::elements::Break::new(0.5));

    let mut project_table = TableLayout::new(vec![3, 2]);
    project_table.set_cell_decorator(genpdfi::elements::FrameCellDecorator::new(
        false, false, false,
    ));

    let header_row = project_table.row();
    let header_row = header_row
        .element(Text::new("Project").styled(Style::new().bold()))
        .element(Text::new("Total Hours").styled(Style::new().bold()));
    header_row
        .push()
        .map_err(|e| AppError::SystemError(format!("Failed to add table header: {e}")))?;

    let mut sorted_summaries: Vec<_> = report.project_summaries.iter().collect();
    sorted_summaries.sort_by(|a, b| a.name.cmp(&b.name));

    for summary in &sorted_summaries {
        let row = project_table.row();
        let row = row
            .element(Text::new(summary.name.clone()))
            .element(Text::new(format_duration(summary.total)));
        row.push()
            .map_err(|e| AppError::SystemError(format!("Failed to add table row: {e}")))?;
    }

    doc.push(project_table);
    doc.push(genpdfi::elements::Break::new(2.0));

    // SECTION 2: Daily Detail
    let mut detail_heading = Paragraph::new("");
    detail_heading.push(bold_string("Daily Detail"));
    doc.push(detail_heading);
    doc.push(genpdfi::elements::Break::new(0.5));

    for section in &report.daily_sections {
        let date_str = section.date.format("%Y-%m-%d").to_string();
        let mut date_heading = Paragraph::new("");
        date_heading.push(bold_string(&date_str));
        doc.push(date_heading);

        let mut table = TableLayout::new(vec![1, 3, 2, 1, 1, 1]);
        table.set_cell_decorator(genpdfi::elements::FrameCellDecorator::new(
            false, false, false,
        ));

        let header_row = table.row();
        let header_row = header_row
            .element(Text::new("ID").styled(Style::new().bold()))
            .element(Text::new("Description").styled(Style::new().bold()))
            .element(Text::new("Project").styled(Style::new().bold()))
            .element(Text::new("Start").styled(Style::new().bold()))
            .element(Text::new("End").styled(Style::new().bold()))
            .element(Text::new("Duration").styled(Style::new().bold()));
        header_row
            .push()
            .map_err(|e| AppError::SystemError(format!("Failed to add table header: {e}")))?;

        for entry in &section.entries {
            let start = entry
                .task
                .start_time
                .map(|t| t.format("%H:%M").to_string())
                .unwrap_or_else(|| "-".to_string());

            let end = entry
                .task
                .end_time
                .map(|t| t.format("%H:%M").to_string())
                .unwrap_or_else(|| "-".to_string());

            let row = table.row();
            let row = row
                .element(Text::new(entry.task.id.to_string()))
                .element(Paragraph::new(entry.task.description.clone()))
                .element(Text::new(entry.project_name.clone()))
                .element(Text::new(start))
                .element(Text::new(end))
                .element(Text::new(format_duration(entry.task.duration_min)));
            row.push()
                .map_err(|e| AppError::SystemError(format!("Failed to add table row: {e}")))?;
        }

        doc.push(table);
        doc.push(genpdfi::elements::Break::new(1.0));
    }

    // Grand total
    doc.push(genpdfi::elements::Break::new(0.5));
    let mut total_para = Paragraph::new("");
    total_para.push(bold_string(&format!(
        "Grand Total: {}",
        format_duration(report.grand_total)
    )));
    doc.push(total_para);

    // Atomic write: temp file -> rename
    let tmp_path = output_path.with_extension("pdf.tmp");
    doc.render_to_file(&tmp_path).map_err(|e| {
        let _ = std::fs::remove_file(&tmp_path);
        AppError::SystemError(format!(
            "Failed to write PDF: {e}. No partial file was created."
        ))
    })?;

    std::fs::rename(&tmp_path, output_path).map_err(|e| {
        let _ = std::fs::remove_file(&tmp_path);
        AppError::SystemError(format!(
            "Failed to write PDF to '{}': {e}",
            output_path.display()
        ))
    })?;

    Ok(())
}

/// Resolve the output path for a PDF file based on CLI flags.
///
/// Returns `Ok(None)` if no PDF generation was requested.
/// Returns `Err` if the output path is invalid (e.g., parent directory doesn't exist).
pub fn resolve_pdf_path(
    output: Option<&str>,
    pdf_flag: bool,
    from: NaiveDate,
    to: NaiveDate,
) -> Result<Option<PathBuf>, AppError> {
    let auto_name = if from == to {
        format!("report-{}.pdf", from.format("%Y-%m-%d"))
    } else {
        format!(
            "report-{}-to-{}.pdf",
            from.format("%Y-%m-%d"),
            to.format("%Y-%m-%d")
        )
    };

    if let Some(out) = output {
        let path = PathBuf::from(out);
        let resolved = if path.extension().is_some_and(|ext| ext == "pdf") {
            path
        } else {
            path.join(&auto_name)
        };

        if let Some(parent) = resolved.parent() {
            if !parent.as_os_str().is_empty() && !parent.exists() {
                return Err(AppError::UserError(format!(
                    "Directory '{}' does not exist.",
                    parent.display()
                )));
            }
        }

        Ok(Some(resolved))
    } else if pdf_flag {
        Ok(Some(PathBuf::from(&auto_name)))
    } else {
        Ok(None)
    }
}
