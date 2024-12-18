use tui::{
    layout::{Constraint, Direction, Layout, Rect},
    text::{Line, Span},
    widgets::{Block, Paragraph},
    Frame,
};

use crate::{
    app::App,
    canvas::{drawing_utils::widget_block, Painter},
};

impl Painter {
    pub fn draw_basic_network(
        &self, f: &mut Frame<'_>, app_state: &mut App, draw_loc: Rect, widget_id: u64,
    ) {
        let divided_loc = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(draw_loc);

        let net_loc = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(100)])
            .horizontal_margin(1)
            .split(divided_loc[0]);

        let total_loc = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(100)])
            .horizontal_margin(1)
            .split(divided_loc[1]);

        if app_state.current_widget.widget_id == widget_id {
            f.render_widget(
                widget_block(true, true, self.styles.border_type)
                    .border_style(self.styles.highlighted_border_style),
                draw_loc,
            );
        }

        let rx_label = format!("RX: {}", app_state.converted_data.rx_display);
        let tx_label = format!("TX: {}", app_state.converted_data.tx_display);
        let total_rx_label = format!("Total RX: {}", app_state.converted_data.total_rx_display);
        let total_tx_label = format!("Total TX: {}", app_state.converted_data.total_tx_display);

        let net_text = vec![
            Line::from(Span::styled(rx_label, self.styles.rx_style)),
            Line::from(Span::styled(tx_label, self.styles.tx_style)),
        ];

        let total_net_text = vec![
            Line::from(Span::styled(total_rx_label, self.styles.total_rx_style)),
            Line::from(Span::styled(total_tx_label, self.styles.total_tx_style)),
        ];

        f.render_widget(Paragraph::new(net_text).block(Block::default()), net_loc[0]);

        f.render_widget(
            Paragraph::new(total_net_text).block(Block::default()),
            total_loc[0],
        );

        // Update draw loc in widget map
        if app_state.should_get_widget_bounds() {
            if let Some(widget) = app_state.widget_map.get_mut(&widget_id) {
                widget.top_left_corner = Some((draw_loc.x, draw_loc.y));
                widget.bottom_right_corner =
                    Some((draw_loc.x + draw_loc.width, draw_loc.y + draw_loc.height));
            }
        }
    }
}
