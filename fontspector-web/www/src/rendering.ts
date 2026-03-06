import { FontInfo } from "./types";
import { SVG } from '@svgdotjs/svg.js'

type Renderer = (metadata: any, font: FontInfo) => [string, boolean];
export const CheckSpecificRendering: Record<string, Renderer> = {};

type AlignmentMisses = {
    glyph_name: string;
    glyph_id: number;
    x: number;
    y: number;
    line: string;
    y_expected: number;
}[];

CheckSpecificRendering["outline_alignment_miss"] = function (metadata, font: FontInfo) {
    let misses: AlignmentMisses = metadata;
    let per_glyph_misses = misses.reduce((acc: Record<string, AlignmentMisses>, miss) => {
        if (!acc[miss.glyph_name]) {
            acc[miss.glyph_name] = [];
        }
        acc[miss.glyph_name].push(miss);
        return acc;
    }, {});
    let reports = "";
    console.log(per_glyph_misses);  
    for (var glyph of Object.keys(per_glyph_misses)) {
        reports += `\n\n<h5>${glyph}</h5>\n`;
        let svg = SVG();
        let misses = per_glyph_misses[glyph];
        let outline = font.font.glyphToPath(misses[0].glyph_id);
        let pathgroup = svg.group()
        pathgroup.path(outline).fill('none').stroke({ color: 'black', width: 1 });
        // Reflect it
        var bounds = svg.bbox();
        
        let width = bounds.width;
        let doneLines: Record<string, boolean> = {};
        let miss_html = "";
        for (var miss of misses) {
            svg.circle(10).fill('red').center(miss.x, miss.y);
            if (!doneLines[miss.line]) {
                doneLines[miss.line] = true;
                svg.line(bounds.x, miss.y_expected, bounds.x2, miss.y_expected).stroke({ color: 'blue', width: 1 });
                svg.text(miss.line).move(bounds.cx, miss.y_expected - 20).font({ size: 20 }).fill('blue').flip("y");
            }
            miss_html += `<li>Point at (${miss.x}, ${miss.y}) should be at ${miss.line} (y=${miss.y_expected})</li>`;
        }
        svg.viewbox(svg.bbox());
        svg.transform({ scaleY: -1 });
        // SVG.js gets too clever here.
        svg.translate(0, -svg.transform().translateY);
        svg.size(200, 200);
        reports += `
            <div class="row">
                <div class="col-sm">
                    <div style="width: 200px; height: 200px;">${svg.svg()}</div>
                </div>
                <div class="col-sm">
                    <ul>${miss_html}</ul>
                </div>
            </div>
        `;
    }
    return [reports, true];
}