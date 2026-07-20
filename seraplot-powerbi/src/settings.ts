"use strict";

import { formattingSettings } from "powerbi-visuals-utils-formattingmodel";

import FormattingSettingsCard = formattingSettings.SimpleCard;
import FormattingSettingsSlice = formattingSettings.Slice;
import FormattingSettingsModel = formattingSettings.Model;

class ChartSettings extends FormattingSettingsCard {
    chartType = new formattingSettings.ItemDropdown({
        name: "chartType",
        displayName: "Chart type",
        items: [
            { value: "bar", displayName: "Bar" },
            { value: "line", displayName: "Line" },
            { value: "pie", displayName: "Pie" },
            { value: "area", displayName: "Area" },
            { value: "histogram", displayName: "Histogram" },
            { value: "scatter", displayName: "Scatter" },
        ],
        value: { value: "bar", displayName: "Bar" },
    });

    backgroundColor = new formattingSettings.ColorPicker({
        name: "backgroundColor",
        displayName: "Background",
        value: { value: "#0d1117" },
    });

    name: string = "chartSettings";
    displayName: string = "SeraPlot chart";
    slices: Array<FormattingSettingsSlice> = [this.chartType, this.backgroundColor];
}

export class VisualFormattingSettingsModel extends FormattingSettingsModel {
    chartSettings = new ChartSettings();

    cards = [this.chartSettings];
}
