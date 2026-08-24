use super::common::pack_local;
use super::config::CirclePackConfig;
use crate::html::hover::{build_chart_html, slots_to_json, HoverSlot};
use crate::plot::statistical::common::{escape_xml, hex6, palette_color, push_b, push_f2, push_i, svg_open};
use std::collections::HashMap;

const R_MIN: f64 = 1.3;
const R_MAX: f64 = 17.0;
const PAD: f64 = 22.0;
const LEGEND_H: f64 = 108.0;
const LEGEND_GAP: f64 = 16.0;
const COLHEAD_H: f64 = 40.0;
const ROWLABEL_W: f64 = 150.0;
const STEP_R: f64 = 9.0;
const INK: &str = "1e293b";
const SUB: &str = "64748b";
const STRONG: &str = "334155";
const RULE: &str = "e2e8f0";
const STAIR: &str = "94a3b8";
const NEUTRAL_DOT: &str = "94a3b8";

struct Sat<'a> {
    name: &'a str,
    region: usize,
    orbit: usize,
    cat: usize,
    sym: &'a str,
    mass: f64,
    shade: Option<f64>,
}

fn ordered_by_count(values: &[String]) -> (Vec<String>, HashMap<String, usize>) {
    let mut counts: Vec<(String, usize)> = Vec::new();
    for v in values {
        if let Some(e) = counts.iter_mut().find(|(k, _)| k == v) {
            e.1 += 1;
        } else {
            counts.push((v.clone(), 1));
        }
    }
    counts.sort_by(|a, b| b.1.cmp(&a.1));
    let order: Vec<String> = counts.into_iter().map(|(k, _)| k).collect();
    let idx: HashMap<String, usize> = order.iter().enumerate().map(|(i, k)| (k.clone(), i)).collect();
    (order, idx)
}

fn proportional_extents(counts: &[usize], total: f64, gap: f64) -> Vec<f64> {
    let n = counts.len();
    if n == 0 {
        return Vec::new();
    }
    let sum = counts.iter().map(|&c| c as f64).sum::<f64>().max(1.0);
    let equal = 1.0 / n as f64;
    let floor = equal * 0.32;
    let mut frac: Vec<f64> = counts.iter().map(|&c| (c as f64 / sum).max(floor)).collect();
    let fsum: f64 = frac.iter().sum();
    for f in frac.iter_mut() {
        *f /= fsum;
    }
    let avail = (total - gap * (n as f64 - 1.0)).max(n as f64 * 6.0);
    frac.into_iter().map(|f| f * avail).collect()
}

fn cap_label(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
        None => String::new(),
    }
}

fn legend_dot(buf: &mut Vec<u8>, x: f64, y: f64, r: f64, hex: &[u8], opacity: f64) {
    push_b(buf, b"<circle cx=\"");
    push_f2(buf, x);
    push_b(buf, b"\" cy=\"");
    push_f2(buf, y);
    push_b(buf, b"\" r=\"");
    push_f2(buf, r.max(1.0));
    push_b(buf, b"\" fill=\"#");
    buf.extend_from_slice(hex);
    push_b(buf, b"\" fill-opacity=\"");
    push_f2(buf, opacity);
    push_b(buf, b"\"/>");
}

fn legend_title(buf: &mut Vec<u8>, x: f64, y: f64, text: &str) {
    push_b(buf, b"<text x=\"");
    push_f2(buf, x);
    push_b(buf, b"\" y=\"");
    push_f2(buf, y);
    push_b(buf, b"\" font-family=\"Arial,sans-serif\" font-size=\"10\" font-weight=\"700\" letter-spacing=\"1\" fill=\"#");
    push_b(buf, SUB.as_bytes());
    push_b(buf, b"\">");
    push_b(buf, text.as_bytes());
    push_b(buf, b"</text>");
}

fn draw_glyph(buf: &mut Vec<u8>, kind: &str, cx: f64, cy: f64, r: f64) {
    let sw = (r * 0.11).clamp(0.55, 1.4);
    match kind {
        "star" => {
            let s = r;
            let inner = s * 0.42;
            push_b(buf, b"<polygon points=\"");
            for k in 0..8 {
                let ang = -std::f64::consts::FRAC_PI_2 + k as f64 * std::f64::consts::FRAC_PI_4;
                let rad = if k % 2 == 0 { s } else { inner };
                if k > 0 {
                    push_b(buf, b" ");
                }
                push_f2(buf, cx + rad * ang.cos());
                push_b(buf, b",");
                push_f2(buf, cy + rad * ang.sin());
            }
            push_b(buf, b"\" fill=\"none\" stroke=\"#ffffff\" stroke-width=\"");
            push_f2(buf, sw);
            push_b(buf, b"\" stroke-linejoin=\"round\"/>");
        }
        "triangle" => {
            let s = r * 0.98;
            push_b(buf, b"<polygon points=\"");
            push_f2(buf, cx);
            push_b(buf, b",");
            push_f2(buf, cy - s);
            push_b(buf, b" ");
            push_f2(buf, cx + s * 0.87);
            push_b(buf, b",");
            push_f2(buf, cy + s * 0.5);
            push_b(buf, b" ");
            push_f2(buf, cx - s * 0.87);
            push_b(buf, b",");
            push_f2(buf, cy + s * 0.5);
            push_b(buf, b"\" fill=\"none\" stroke=\"#ffffff\" stroke-width=\"");
            push_f2(buf, sw);
            push_b(buf, b"\" stroke-linejoin=\"round\"/>");
        }
        "diamond" => {
            let s = r * 0.92;
            push_b(buf, b"<polygon points=\"");
            push_f2(buf, cx);
            push_b(buf, b",");
            push_f2(buf, cy - s);
            push_b(buf, b" ");
            push_f2(buf, cx + s);
            push_b(buf, b",");
            push_f2(buf, cy);
            push_b(buf, b" ");
            push_f2(buf, cx);
            push_b(buf, b",");
            push_f2(buf, cy + s);
            push_b(buf, b" ");
            push_f2(buf, cx - s);
            push_b(buf, b",");
            push_f2(buf, cy);
            push_b(buf, b"\" fill=\"none\" stroke=\"#ffffff\" stroke-width=\"");
            push_f2(buf, sw);
            push_b(buf, b"\" stroke-linejoin=\"round\"/>");
        }
        _ => {}
    }
}

fn staircase_path(buf: &mut Vec<u8>, col_x: &[f64], col_w: &[f64], ys: &[f64]) {
    let n = col_x.len();
    if n == 0 {
        return;
    }
    push_b(buf, b"<path d=\"M");
    push_f2(buf, col_x[0]);
    push_b(buf, b" ");
    push_f2(buf, ys[0]);
    for c in 0..n {
        let x_end = col_x[c] + col_w[c];
        let next_y = if c + 1 < n { ys[c + 1] } else { ys[c] };
        let dy = next_y - ys[c];
        if dy.abs() < 0.6 || c + 1 == n {
            push_b(buf, b" L");
            push_f2(buf, x_end);
            push_b(buf, b" ");
            push_f2(buf, ys[c]);
        } else {
            let cr = STEP_R.min((next_y - ys[c]).abs() * 0.45).max(1.5);
            let sgn = dy.signum();
            push_b(buf, b" L");
            push_f2(buf, x_end - cr);
            push_b(buf, b" ");
            push_f2(buf, ys[c]);
            push_b(buf, b" Q");
            push_f2(buf, x_end);
            push_b(buf, b" ");
            push_f2(buf, ys[c]);
            push_b(buf, b" ");
            push_f2(buf, x_end);
            push_b(buf, b" ");
            push_f2(buf, ys[c] + cr * sgn);
            push_b(buf, b" L");
            push_f2(buf, x_end);
            push_b(buf, b" ");
            push_f2(buf, next_y - cr * sgn);
            push_b(buf, b" Q");
            push_f2(buf, x_end);
            push_b(buf, b" ");
            push_f2(buf, next_y);
            push_b(buf, b" ");
            push_f2(buf, x_end + cr);
            push_b(buf, b" ");
            push_f2(buf, next_y);
        }
    }
    push_b(buf, b"\" fill=\"none\" stroke=\"#");
    push_b(buf, STAIR.as_bytes());
    push_b(buf, b"\" stroke-width=\"1.6\" stroke-linecap=\"round\"/>");
}

#[crate::chart_demo("labels=[\"Lyra-190\",\"Nimbus-552\",\"Comet-631\",\"Terra-31\",\"Halo-5\",\"Vega-261\",\"Orbis-867\",\"Halo-385\",\"Kepler-90\",\"Orbis-12\",\"Comet-788\",\"Orion-995\",\"Ionos-591\",\"Cygnus-304\",\"Meridian-638\",\"Ionos-158\",\"Atlas-202\",\"Helio-525\",\"Halo-301\",\"Orion-631\",\"Zenith-457\",\"Meridian-142\",\"Nova-36\",\"Ionos-77\",\"Meridian-755\",\"Meridian-87\",\"Nova-15\",\"Orbis-264\",\"Argus-859\",\"Draco-981\",\"Meridian-309\",\"Halo-914\",\"Echo-206\",\"Meridian-858\",\"Argus-483\",\"Vega-860\",\"Helio-695\",\"Pulsar-466\",\"Orbis-458\",\"Lyra-844\",\"Draco-279\",\"Pulsar-870\",\"Argus-465\",\"Sentinel-840\",\"Echo-735\",\"Echo-329\",\"Atlas-432\",\"Corvus-546\",\"Rigel-861\",\"Draco-450\",\"Solis-320\",\"Atlas-537\",\"Aster-552\",\"Nimbus-708\",\"Solis-253\",\"Rigel-325\",\"Helio-199\",\"Orbis-512\",\"Titan-448\",\"Atlas-654\",\"Ionos-830\",\"Lyra-909\",\"Cygnus-964\",\"Orbis-206\",\"Beacon-352\",\"Beacon-565\",\"Cygnus-162\",\"Nimbus-784\",\"Aster-650\",\"Echo-200\",\"Titan-882\",\"Sentinel-781\",\"Vela-919\",\"Comet-190\",\"Titan-639\",\"Nova-187\",\"Pulsar-589\",\"Beacon-201\",\"Rigel-426\",\"Atlas-612\",\"Orion-508\",\"Terra-320\",\"Comet-304\",\"Pulsar-55\",\"Sentinel-928\",\"Echo-830\",\"Helio-640\",\"Argus-144\",\"Comet-544\",\"Titan-317\",\"Vega-606\",\"Echo-201\",\"Helio-105\",\"Ionos-383\",\"Sentinel-123\",\"Solis-400\",\"Beacon-765\",\"Nimbus-557\",\"Aster-78\",\"Vega-336\",\"Kepler-727\",\"Sentinel-313\",\"Meridian-849\",\"Meridian-817\",\"Titan-876\",\"Ionos-862\",\"Solis-755\",\"Zenith-252\",\"Kepler-827\",\"Terra-937\",\"Cygnus-92\",\"Zenith-946\",\"Kepler-210\",\"Orbis-311\",\"Halo-932\",\"Vega-484\",\"Echo-320\",\"Echo-979\",\"Lyra-971\",\"Helio-674\",\"Vega-432\",\"Vela-653\",\"Aster-210\",\"Rigel-100\",\"Astra-20\",\"Helio-557\",\"Nimbus-570\",\"Quasar-404\",\"Vega-728\",\"Lyra-924\",\"Beacon-402\",\"Solis-577\",\"Orbis-135\",\"Vega-93\",\"Echo-844\",\"Halo-340\",\"Sentinel-251\",\"Argus-109\",\"Corvus-260\",\"Nova-731\",\"Aster-643\",\"Kepler-94\",\"Draco-926\",\"Titan-838\",\"Draco-777\",\"Pulsar-185\",\"Echo-391\",\"Vela-541\",\"Echo-341\",\"Terra-105\",\"Astra-498\",\"Nova-118\",\"Echo-56\",\"Astra-904\",\"Helio-488\",\"Titan-71\",\"Sentinel-169\",\"Vela-127\",\"Kepler-177\",\"Draco-47\",\"Beacon-484\",\"Cygnus-880\",\"Atlas-648\",\"Titan-281\",\"Zenith-81\",\"Draco-514\",\"Rigel-235\",\"Nimbus-474\",\"Nova-292\",\"Draco-184\",\"Terra-473\",\"Atlas-796\",\"Cygnus-866\",\"Corvus-115\",\"Comet-242\",\"Terra-994\",\"Terra-509\",\"Echo-183\",\"Zenith-44\",\"Nimbus-666\",\"Sentinel-466\",\"Vela-502\",\"Aster-247\",\"Helio-697\",\"Orion-248\",\"Lyra-41\",\"Echo-908\",\"Lyra-429\",\"Terra-479\",\"Lyra-554\",\"Rigel-187\",\"Meridian-59\",\"Aster-330\",\"Quasar-57\",\"Helio-214\",\"Kepler-743\",\"Pulsar-140\",\"Vela-42\",\"Astra-677\",\"Orion-384\",\"Halo-393\",\"Halo-787\",\"Sentinel-155\",\"Orbis-374\",\"Halo-737\",\"Astra-269\",\"Aster-590\",\"Zenith-66\",\"Sentinel-140\",\"Lyra-277\",\"Aster-246\",\"Sentinel-342\",\"Quasar-161\",\"Astra-379\",\"Kepler-724\",\"Titan-722\",\"Orion-423\",\"Solis-734\",\"Cygnus-578\",\"Sentinel-165\",\"Halo-600\",\"Ionos-682\",\"Orion-44\",\"Cygnus-556\",\"Astra-230\",\"Lyra-560\",\"Titan-500\",\"Ionos-410\",\"Vela-66\",\"Sentinel-208\",\"Helio-221\",\"Sentinel-454\",\"Orbis-692\",\"Titan-216\",\"Halo-409\",\"Pulsar-861\",\"Kepler-398\",\"Meridian-320\",\"Corvus-217\",\"Vela-382\",\"Atlas-861\",\"Nova-330\",\"Cygnus-280\",\"Aster-152\",\"Titan-451\",\"Vega-763\",\"Atlas-587\",\"Terra-56\",\"Lyra-755\",\"Orbis-240\",\"Rigel-258\",\"Argus-213\",\"Orbis-569\",\"Beacon-891\",\"Kepler-38\",\"Pulsar-32\",\"Atlas-645\",\"Aster-837\",\"Lyra-186\",\"Meridian-625\",\"Meridian-892\",\"Zenith-286\",\"Halo-594\",\"Argus-232\",\"Meridian-430\",\"Quasar-626\",\"Solis-350\",\"Titan-215\",\"Titan-836\",\"Rigel-324\",\"Pulsar-444\",\"Ionos-411\",\"Rigel-405\",\"Comet-928\",\"Vela-17\",\"Atlas-575\",\"Nimbus-582\",\"Draco-605\",\"Vela-872\",\"Zenith-633\",\"Sentinel-839\",\"Orbis-506\",\"Astra-81\",\"Ionos-908\",\"Astra-274\",\"Ionos-441\",\"Atlas-331\",\"Draco-533\",\"Vega-608\",\"Terra-781\",\"Orion-96\",\"Pulsar-92\",\"Helio-325\",\"Helio-405\",\"Beacon-217\",\"Corvus-801\",\"Argus-328\",\"Vega-116\",\"Orbis-425\",\"Helio-919\",\"Sentinel-398\",\"Lyra-16\",\"Argus-589\",\"Kepler-93\",\"Echo-66\",\"Atlas-185\",\"Terra-105\",\"Atlas-502\",\"Draco-803\",\"Beacon-195\",\"Vega-827\",\"Corvus-551\",\"Solis-30\",\"Orion-135\",\"Beacon-654\",\"Halo-464\",\"Meridian-907\",\"Astra-924\",\"Orion-145\",\"Orion-999\",\"Vega-363\",\"Nimbus-711\",\"Solis-344\",\"Orion-438\",\"Titan-628\",\"Orbis-306\",\"Terra-533\",\"Lyra-999\",\"Draco-269\",\"Draco-954\",\"Echo-341\",\"Titan-23\",\"Helio-810\",\"Zenith-571\",\"Vela-430\",\"Atlas-608\",\"Pulsar-978\",\"Echo-327\",\"Draco-139\",\"Meridian-745\",\"Quasar-51\",\"Argus-781\",\"Beacon-957\",\"Halo-812\",\"Lyra-151\",\"Astra-167\",\"Sentinel-985\",\"Helio-345\",\"Corvus-366\",\"Kepler-973\",\"Argus-522\",\"Nova-414\",\"Halo-573\",\"Halo-411\",\"Ionos-494\",\"Sentinel-52\",\"Orbis-437\",\"Cygnus-682\",\"Quasar-940\",\"Orbis-835\",\"Kepler-483\",\"Quasar-164\",\"Beacon-777\",\"Orion-418\",\"Echo-504\",\"Draco-670\",\"Corvus-28\",\"Astra-546\",\"Lyra-536\",\"Comet-539\",\"Kepler-54\",\"Zenith-943\",\"Zenith-781\",\"Atlas-430\",\"Corvus-565\",\"Helio-723\",\"Titan-927\",\"Zenith-511\",\"Zenith-48\",\"Nimbus-761\",\"Nova-895\",\"Rigel-591\",\"Solis-460\",\"Zenith-894\",\"Quasar-386\",\"Comet-233\",\"Lyra-670\",\"Vega-401\",\"Orion-408\",\"Cygnus-221\",\"Meridian-593\",\"Vela-771\",\"Kepler-541\",\"Astra-928\",\"Atlas-451\",\"Atlas-709\",\"Atlas-526\",\"Draco-212\",\"Aster-501\",\"Vega-372\",\"Aster-453\",\"Draco-617\",\"Corvus-848\",\"Lyra-789\",\"Quasar-49\",\"Orbis-332\",\"Sentinel-105\",\"Vega-695\",\"Aster-255\",\"Solis-178\",\"Orion-638\",\"Rigel-272\",\"Astra-23\",\"Lyra-377\",\"Nimbus-407\",\"Nimbus-113\",\"Rigel-366\",\"Vega-655\",\"Nova-58\",\"Echo-458\",\"Draco-739\",\"Rigel-104\",\"Helio-353\",\"Titan-485\",\"Ionos-118\",\"Solis-397\",\"Solis-940\",\"Vega-123\",\"Aster-61\",\"Terra-381\",\"Rigel-12\",\"Helio-80\",\"Terra-385\",\"Solis-167\",\"Helio-143\",\"Corvus-196\",\"Sentinel-385\",\"Helio-630\",\"Astra-555\",\"Orbis-931\",\"Vega-752\",\"Atlas-124\",\"Titan-912\",\"Pulsar-509\",\"Lyra-331\",\"Solis-192\",\"Terra-348\",\"Rigel-653\",\"Orion-486\",\"Sentinel-475\",\"Terra-614\",\"Quasar-691\",\"Cygnus-263\",\"Ionos-27\",\"Vela-96\",\"Meridian-906\",\"Astra-455\",\"Halo-616\",\"Quasar-100\",\"Pulsar-984\",\"Helio-719\",\"Orbis-617\",\"Comet-12\",\"Comet-66\",\"Echo-50\",\"Kepler-482\",\"Halo-507\",\"Argus-633\",\"Draco-144\",\"Kepler-896\",\"Echo-746\",\"Halo-867\",\"Draco-132\",\"Titan-802\",\"Orbis-663\",\"Cygnus-295\",\"Echo-934\",\"Nimbus-547\",\"Lyra-820\",\"Nova-822\",\"Terra-568\",\"Argus-363\",\"Vela-350\",\"Terra-161\",\"Ionos-37\",\"Sentinel-528\",\"Nimbus-838\",\"Vela-477\",\"Sentinel-429\",\"Ionos-788\",\"Argus-43\",\"Atlas-596\",\"Cygnus-677\",\"Ionos-124\",\"Echo-362\",\"Aster-480\",\"Vega-149\",\"Quasar-965\",\"Vela-61\",\"Argus-784\",\"Kepler-482\",\"Orbis-813\",\"Orbis-176\",\"Nimbus-860\",\"Helio-303\",\"Nimbus-422\",\"Astra-423\",\"Quasar-47\",\"Beacon-306\",\"Meridian-8\",\"Ionos-506\",\"Titan-807\",\"Rigel-839\",\"Echo-910\",\"Solis-148\",\"Aster-337\",\"Zenith-727\",\"Halo-255\",\"Vela-280\",\"Zenith-137\",\"Vega-251\",\"Astra-669\",\"Quasar-243\",\"Solis-995\",\"Solis-979\",\"Corvus-232\",\"Cygnus-343\",\"Pulsar-63\",\"Lyra-682\",\"Comet-733\",\"Vega-112\",\"Cygnus-712\",\"Comet-315\",\"Orion-698\",\"Argus-796\",\"Helio-5\",\"Beacon-615\",\"Echo-360\",\"Astra-763\",\"Terra-701\",\"Astra-348\",\"Titan-292\",\"Comet-387\",\"Terra-162\",\"Rigel-988\",\"Orbis-224\",\"Lyra-911\",\"Halo-102\",\"Vela-63\",\"Vega-276\",\"Corvus-719\",\"Orion-295\",\"Orbis-987\",\"Vega-627\",\"Vela-869\",\"Astra-847\",\"Helio-250\",\"Astra-160\",\"Orion-932\",\"Orion-365\",\"Atlas-306\",\"Nova-403\",\"Vela-712\",\"Rigel-711\",\"Titan-738\",\"Aster-249\",\"Ionos-15\",\"Vela-600\",\"Titan-828\",\"Kepler-354\",\"Zenith-788\",\"Astra-632\",\"Nova-548\",\"Nimbus-874\",\"Zenith-844\",\"Orbis-141\",\"Echo-860\",\"Zenith-99\",\"Meridian-419\",\"Sentinel-848\",\"Lyra-934\",\"Solis-762\",\"Aster-264\",\"Draco-69\",\"Titan-955\",\"Terra-238\",\"Pulsar-885\",\"Orion-825\",\"Sentinel-566\",\"Aster-589\",\"Draco-699\",\"Nova-659\",\"Argus-538\",\"Solis-563\",\"Atlas-190\",\"Pulsar-450\",\"Beacon-447\",\"Solis-93\",\"Corvus-402\",\"Corvus-175\",\"Lyra-931\",\"Argus-293\",\"Nova-614\",\"Orion-604\",\"Lyra-449\",\"Sentinel-956\",\"Zenith-730\",\"Orion-449\",\"Vela-761\",\"Aster-808\",\"Quasar-55\",\"Nova-110\",\"Zenith-947\",\"Nova-60\",\"Aster-706\",\"Argus-201\",\"Orbis-87\",\"Vela-474\",\"Beacon-859\",\"Rigel-229\",\"Quasar-464\",\"Echo-334\",\"Meridian-7\",\"Aster-254\",\"Atlas-732\",\"Nimbus-757\",\"Aster-140\",\"Corvus-47\",\"Argus-521\",\"Lyra-295\",\"Halo-421\",\"Titan-707\",\"Terra-155\",\"Aster-908\",\"Aster-787\",\"Ionos-291\",\"Solis-11\",\"Sentinel-698\",\"Quasar-536\",\"Draco-508\",\"Helio-595\",\"Nova-585\",\"Quasar-507\",\"Vela-876\",\"Nova-429\",\"Vega-413\",\"Comet-866\",\"Vela-309\",\"Quasar-96\",\"Titan-107\",\"Argus-71\",\"Nimbus-815\",\"Kepler-241\",\"Astra-873\",\"Cygnus-338\",\"Helio-643\",\"Quasar-538\",\"Vela-693\",\"Astra-29\",\"Atlas-894\",\"Vela-751\",\"Atlas-44\",\"Sentinel-324\",\"Pulsar-985\",\"Nova-447\",\"Lyra-844\",\"Terra-171\",\"Comet-397\",\"Vega-751\",\"Echo-829\",\"Echo-814\",\"Halo-975\",\"Corvus-893\",\"Zenith-950\",\"Sentinel-316\",\"Comet-305\",\"Comet-751\",\"Draco-265\",\"Halo-11\",\"Comet-229\",\"Nova-835\",\"Argus-7\",\"Solis-450\",\"Echo-983\",\"Argus-980\",\"Helio-949\",\"Argus-25\",\"Lyra-30\",\"Rigel-89\",\"Quasar-283\",\"Comet-386\",\"Aster-273\",\"Beacon-944\",\"Orbis-390\",\"Lyra-464\",\"Orbis-695\",\"Echo-791\",\"Halo-580\",\"Terra-50\",\"Terra-590\",\"Aster-616\",\"Terra-452\",\"Pulsar-521\",\"Halo-862\",\"Echo-442\",\"Halo-183\",\"Draco-613\",\"Sentinel-318\",\"Draco-498\",\"Vela-463\",\"Beacon-69\",\"Echo-480\",\"Orbis-361\",\"Vega-32\",\"Meridian-710\",\"Argus-547\",\"Vega-375\",\"Comet-481\",\"Terra-849\",\"Solis-833\",\"Draco-387\",\"Draco-601\",\"Terra-27\",\"Orbis-864\",\"Corvus-293\",\"Argus-835\",\"Sentinel-90\",\"Echo-964\",\"Lyra-472\",\"Nova-329\",\"Nova-175\",\"Rigel-631\",\"Quasar-141\",\"Ionos-8\",\"Beacon-718\",\"Helio-289\",\"Meridian-515\",\"Halo-857\",\"Argus-742\",\"Lyra-353\",\"Quasar-944\",\"Cygnus-908\",\"Kepler-359\",\"Orion-344\",\"Kepler-755\",\"Helio-414\",\"Helio-16\",\"Astra-678\",\"Halo-91\",\"Echo-358\",\"Cygnus-819\",\"Zenith-438\",\"Argus-891\",\"Meridian-835\",\"Sentinel-457\",\"Draco-969\",\"Titan-306\",\"Orbis-741\",\"Lyra-143\",\"Orion-988\",\"Beacon-108\",\"Kepler-365\",\"Kepler-627\",\"Draco-468\",\"Orion-99\",\"Terra-539\",\"Pulsar-553\",\"Orbis-77\",\"Orion-713\",\"Orion-351\",\"Ionos-265\",\"Argus-667\",\"Meridian-614\",\"Atlas-732\",\"Cygnus-299\",\"Sentinel-613\",\"Sentinel-34\",\"Nimbus-32\",\"Aster-901\",\"Orion-326\",\"Sentinel-669\",\"Draco-459\",\"Beacon-719\",\"Aster-115\",\"Helio-932\",\"Nimbus-894\",\"Orion-988\",\"Corvus-872\",\"Halo-930\",\"Atlas-439\",\"Terra-934\",\"Kepler-409\",\"Helio-474\",\"Orbis-396\",\"Beacon-441\",\"Argus-708\",\"Vela-16\",\"Titan-378\",\"Orion-646\",\"Sentinel-379\",\"Draco-880\",\"Argus-790\",\"Ionos-529\",\"Helio-209\",\"Comet-677\",\"Zenith-599\",\"Argus-505\",\"Aster-380\",\"Nimbus-432\",\"Quasar-298\",\"Kepler-606\",\"Halo-369\",\"Echo-313\",\"Atlas-522\",\"Solis-465\",\"Pulsar-439\",\"Nimbus-663\",\"Nimbus-623\",\"Pulsar-723\",\"Argus-53\",\"Rigel-995\",\"Vela-842\",\"Vega-715\",\"Cygnus-781\",\"Corvus-372\",\"Corvus-359\",\"Ionos-966\",\"Orion-600\",\"Atlas-606\",\"Solis-384\",\"Lyra-858\",\"Rigel-743\",\"Atlas-40\",\"Zenith-765\",\"Titan-435\",\"Rigel-772\",\"Halo-143\",\"Nimbus-363\",\"Sentinel-671\",\"Halo-577\",\"Quasar-664\",\"Terra-932\",\"Terra-744\",\"Orbis-308\",\"Terra-446\",\"Orion-559\",\"Argus-18\",\"Sentinel-680\",\"Astra-296\",\"Vega-241\",\"Kepler-665\",\"Halo-622\",\"Sentinel-544\",\"Pulsar-750\",\"Vega-110\",\"Nova-215\",\"Atlas-498\",\"Orbis-3\",\"Vega-252\",\"Nova-666\",\"Corvus-458\",\"Nova-62\",\"Halo-520\",\"Nova-757\",\"Cygnus-299\",\"Pulsar-57\",\"Kepler-252\",\"Comet-689\",\"Zenith-752\",\"Rigel-359\",\"Comet-727\",\"Zenith-714\",\"Aster-536\",\"Orion-595\",\"Titan-431\",\"Echo-455\",\"Echo-941\",\"Argus-252\",\"Helio-183\",\"Titan-805\",\"Corvus-892\",\"Corvus-512\",\"Sentinel-790\",\"Lyra-903\",\"Beacon-474\",\"Lyra-163\",\"Rigel-972\",\"Pulsar-455\",\"Cygnus-887\",\"Orion-932\",\"Kepler-405\",\"Helio-975\",\"Corvus-821\",\"Helio-535\",\"Quasar-168\",\"Vela-237\",\"Nimbus-88\",\"Meridian-262\",\"Titan-951\",\"Lyra-729\",\"Lyra-171\",\"Draco-279\",\"Pulsar-348\",\"Vela-251\",\"Corvus-952\",\"Zenith-109\",\"Sentinel-300\",\"Comet-642\",\"Orion-419\",\"Aster-882\",\"Orbis-774\",\"Nimbus-394\",\"Nimbus-907\",\"Helio-416\",\"Orbis-509\",\"Comet-475\",\"Beacon-265\",\"Beacon-943\",\"Astra-187\",\"Zenith-391\",\"Aster-483\",\"Orion-643\",\"Lyra-673\",\"Solis-965\",\"Lyra-951\",\"Zenith-256\",\"Comet-98\",\"Argus-709\",\"Solis-274\",\"Aster-18\",\"Lyra-692\",\"Zenith-220\",\"Aster-825\",\"Ionos-60\",\"Ionos-737\",\"Nimbus-745\",\"Corvus-67\",\"Comet-714\",\"Draco-477\",\"Beacon-246\",\"Nimbus-905\",\"Titan-449\",\"Corvus-633\",\"Rigel-665\",\"Echo-220\",\"Kepler-8\",\"Ionos-182\",\"Meridian-890\",\"Aster-826\",\"Vela-269\"], parents=[\"China\",\"Japan\",\"Japan\",\"W. Europe\",\"Japan\",\"W. Europe\",\"U.S.\",\"U.S.\",\"Other\",\"U.S.\",\"U.S.\",\"U.S.\",\"Other\",\"U.S.\",\"China\",\"China\",\"U.S.\",\"W. Europe\",\"Other\",\"Russia\",\"W. Europe\",\"U.S.\",\"Other\",\"Other\",\"China\",\"U.S.\",\"W. Europe\",\"W. Europe\",\"India\",\"Russia\",\"W. Europe\",\"U.S.\",\"U.S.\",\"India\",\"Other\",\"U.S.\",\"India\",\"W. Europe\",\"U.S.\",\"U.S.\",\"China\",\"Japan\",\"Japan\",\"U.S.\",\"Other\",\"U.S.\",\"U.S.\",\"Japan\",\"U.S.\",\"W. Europe\",\"Russia\",\"W. Europe\",\"U.S.\",\"Other\",\"U.S.\",\"China\",\"U.S.\",\"U.S.\",\"W. Europe\",\"U.S.\",\"U.S.\",\"China\",\"U.S.\",\"China\",\"Russia\",\"Russia\",\"W. Europe\",\"Other\",\"U.S.\",\"W. Europe\",\"W. Europe\",\"Other\",\"U.S.\",\"U.S.\",\"U.S.\",\"U.S.\",\"Russia\",\"U.S.\",\"W. Europe\",\"Other\",\"W. Europe\",\"Russia\",\"China\",\"U.S.\",\"Other\",\"U.S.\",\"W. Europe\",\"China\",\"W. Europe\",\"Japan\",\"Japan\",\"U.S.\",\"India\",\"U.S.\",\"China\",\"China\",\"Japan\",\"India\",\"U.S.\",\"U.S.\",\"U.S.\",\"China\",\"China\",\"China\",\"China\",\"U.S.\",\"Japan\",\"U.S.\",\"W. Europe\",\"Other\",\"U.S.\",\"India\",\"China\",\"China\",\"W. Europe\",\"Japan\",\"Russia\",\"W. Europe\",\"China\",\"U.S.\",\"Other\",\"W. Europe\",\"Japan\",\"India\",\"India\",\"Japan\",\"China\",\"U.S.\",\"W. Europe\",\"China\",\"W. Europe\",\"Russia\",\"China\",\"W. Europe\",\"U.S.\",\"China\",\"Other\",\"W. Europe\",\"Russia\",\"Japan\",\"U.S.\",\"W. Europe\",\"Other\",\"Russia\",\"U.S.\",\"Russia\",\"Japan\",\"W. Europe\",\"China\",\"U.S.\",\"China\",\"India\",\"Other\",\"U.S.\",\"Russia\",\"W. Europe\",\"Other\",\"U.S.\",\"India\",\"U.S.\",\"U.S.\",\"India\",\"China\",\"U.S.\",\"India\",\"U.S.\",\"W. Europe\",\"Other\",\"Other\",\"Russia\",\"U.S.\",\"Other\",\"W. Europe\",\"W. Europe\",\"W. Europe\",\"U.S.\",\"W. Europe\",\"Japan\",\"Russia\",\"W. Europe\",\"India\",\"U.S.\",\"Japan\",\"China\",\"Other\",\"China\",\"U.S.\",\"U.S.\",\"China\",\"Japan\",\"China\",\"Japan\",\"U.S.\",\"India\",\"U.S.\",\"U.S.\",\"China\",\"China\",\"India\",\"W. Europe\",\"U.S.\",\"Other\",\"U.S.\",\"China\",\"Other\",\"U.S.\",\"Japan\",\"W. Europe\",\"U.S.\",\"W. Europe\",\"U.S.\",\"Russia\",\"U.S.\",\"Other\",\"China\",\"U.S.\",\"U.S.\",\"India\",\"W. Europe\",\"India\",\"U.S.\",\"W. Europe\",\"China\",\"U.S.\",\"India\",\"China\",\"U.S.\",\"China\",\"U.S.\",\"U.S.\",\"Other\",\"Russia\",\"Russia\",\"U.S.\",\"U.S.\",\"U.S.\",\"W. Europe\",\"W. Europe\",\"U.S.\",\"U.S.\",\"China\",\"Other\",\"China\",\"U.S.\",\"W. Europe\",\"India\",\"China\",\"U.S.\",\"Other\",\"China\",\"U.S.\",\"Other\",\"Russia\",\"China\",\"Russia\",\"Other\",\"India\",\"U.S.\",\"U.S.\",\"Japan\",\"W. Europe\",\"U.S.\",\"China\",\"U.S.\",\"Other\",\"U.S.\",\"U.S.\",\"U.S.\",\"U.S.\",\"Other\",\"China\",\"India\",\"Japan\",\"U.S.\",\"U.S.\",\"China\",\"Russia\",\"U.S.\",\"China\",\"Other\",\"W. Europe\",\"U.S.\",\"U.S.\",\"U.S.\",\"Russia\",\"W. Europe\",\"U.S.\",\"China\",\"Japan\",\"China\",\"Russia\",\"U.S.\",\"U.S.\",\"U.S.\",\"W. Europe\",\"Russia\",\"U.S.\",\"Other\",\"W. Europe\",\"U.S.\",\"China\",\"Russia\",\"China\",\"Russia\",\"W. Europe\",\"Japan\",\"Other\",\"W. Europe\",\"U.S.\",\"U.S.\",\"Russia\",\"U.S.\",\"India\",\"U.S.\",\"W. Europe\",\"U.S.\",\"W. Europe\",\"U.S.\",\"U.S.\",\"U.S.\",\"Japan\",\"W. Europe\",\"China\",\"U.S.\",\"Other\",\"U.S.\",\"India\",\"India\",\"China\",\"U.S.\",\"India\",\"U.S.\",\"U.S.\",\"U.S.\",\"Other\",\"Japan\",\"Other\",\"U.S.\",\"China\",\"W. Europe\",\"U.S.\",\"U.S.\",\"Japan\",\"Other\",\"Other\",\"Other\",\"U.S.\",\"China\",\"U.S.\",\"U.S.\",\"Other\",\"Russia\",\"U.S.\",\"U.S.\",\"U.S.\",\"U.S.\",\"U.S.\",\"China\",\"Russia\",\"U.S.\",\"China\",\"Japan\",\"U.S.\",\"U.S.\",\"Japan\",\"U.S.\",\"W. Europe\",\"W. Europe\",\"Other\",\"Other\",\"U.S.\",\"Other\",\"U.S.\",\"W. Europe\",\"Other\",\"Other\",\"U.S.\",\"U.S.\",\"Russia\",\"U.S.\",\"China\",\"U.S.\",\"China\",\"U.S.\",\"U.S.\",\"W. Europe\",\"U.S.\",\"U.S.\",\"U.S.\",\"U.S.\",\"India\",\"China\",\"Japan\",\"Other\",\"Japan\",\"Russia\",\"U.S.\",\"U.S.\",\"U.S.\",\"U.S.\",\"W. Europe\",\"Other\",\"W. Europe\",\"U.S.\",\"India\",\"India\",\"U.S.\",\"China\",\"U.S.\",\"Other\",\"U.S.\",\"Japan\",\"China\",\"U.S.\",\"India\",\"Russia\",\"U.S.\",\"Other\",\"Japan\",\"Russia\",\"W. Europe\",\"W. Europe\",\"Other\",\"U.S.\",\"W. Europe\",\"W. Europe\",\"W. Europe\",\"U.S.\",\"Other\",\"China\",\"Russia\",\"China\",\"Russia\",\"Japan\",\"W. Europe\",\"India\",\"China\",\"China\",\"U.S.\",\"U.S.\",\"W. Europe\",\"Other\",\"Japan\",\"U.S.\",\"Russia\",\"Other\",\"China\",\"W. Europe\",\"U.S.\",\"Other\",\"Russia\",\"U.S.\",\"China\",\"W. Europe\",\"China\",\"W. Europe\",\"China\",\"Japan\",\"Other\",\"Other\",\"Other\",\"China\",\"U.S.\",\"U.S.\",\"Russia\",\"U.S.\",\"China\",\"U.S.\",\"U.S.\",\"India\",\"U.S.\",\"U.S.\",\"W. Europe\",\"U.S.\",\"Other\",\"U.S.\",\"China\",\"U.S.\",\"U.S.\",\"U.S.\",\"Japan\",\"U.S.\",\"U.S.\",\"Japan\",\"U.S.\",\"China\",\"Japan\",\"U.S.\",\"U.S.\",\"U.S.\",\"W. Europe\",\"U.S.\",\"China\",\"Other\",\"China\",\"U.S.\",\"Japan\",\"U.S.\",\"U.S.\",\"Russia\",\"Japan\",\"Japan\",\"Other\",\"India\",\"Russia\",\"U.S.\",\"U.S.\",\"U.S.\",\"U.S.\",\"U.S.\",\"China\",\"U.S.\",\"W. Europe\",\"Other\",\"U.S.\",\"Other\",\"U.S.\",\"China\",\"U.S.\",\"Russia\",\"Russia\",\"U.S.\",\"U.S.\",\"U.S.\",\"Other\",\"Russia\",\"Japan\",\"Russia\",\"Russia\",\"U.S.\",\"Japan\",\"U.S.\",\"Russia\",\"W. Europe\",\"Russia\",\"U.S.\",\"W. Europe\",\"Other\",\"U.S.\",\"China\",\"U.S.\",\"China\",\"Other\",\"U.S.\",\"India\",\"U.S.\",\"U.S.\",\"Japan\",\"Russia\",\"India\",\"U.S.\",\"China\",\"W. Europe\",\"U.S.\",\"U.S.\",\"U.S.\",\"U.S.\",\"W. Europe\",\"Russia\",\"U.S.\",\"U.S.\",\"U.S.\",\"India\",\"China\",\"China\",\"China\",\"U.S.\",\"U.S.\",\"U.S.\",\"W. Europe\",\"U.S.\",\"India\",\"Russia\",\"U.S.\",\"China\",\"U.S.\",\"U.S.\",\"W. Europe\",\"India\",\"India\",\"India\",\"Other\",\"India\",\"Japan\",\"U.S.\",\"U.S.\",\"Russia\",\"U.S.\",\"Russia\",\"Russia\",\"China\",\"U.S.\",\"W. Europe\",\"W. Europe\",\"Japan\",\"China\",\"U.S.\",\"Other\",\"India\",\"Russia\",\"Russia\",\"Other\",\"India\",\"China\",\"U.S.\",\"U.S.\",\"Japan\",\"Other\",\"China\",\"U.S.\",\"U.S.\",\"China\",\"Other\",\"China\",\"U.S.\",\"Other\",\"Russia\",\"U.S.\",\"W. Europe\",\"China\",\"U.S.\",\"U.S.\",\"W. Europe\",\"Japan\",\"U.S.\",\"U.S.\",\"China\",\"Other\",\"Russia\",\"U.S.\",\"Russia\",\"China\",\"Other\",\"Russia\",\"U.S.\",\"Russia\",\"U.S.\",\"Japan\",\"U.S.\",\"India\",\"U.S.\",\"W. Europe\",\"India\",\"U.S.\",\"Russia\",\"U.S.\",\"India\",\"W. Europe\",\"China\",\"U.S.\",\"W. Europe\",\"U.S.\",\"China\",\"W. Europe\",\"Other\",\"W. Europe\",\"U.S.\",\"Japan\",\"China\",\"W. Europe\",\"U.S.\",\"U.S.\",\"U.S.\",\"Russia\",\"Japan\",\"Japan\",\"China\",\"Other\",\"China\",\"U.S.\",\"W. Europe\",\"China\",\"U.S.\",\"U.S.\",\"China\",\"Other\",\"U.S.\",\"Russia\",\"U.S.\",\"India\",\"Japan\",\"U.S.\",\"China\",\"Other\",\"W. Europe\",\"U.S.\",\"Japan\",\"U.S.\",\"W. Europe\",\"U.S.\",\"U.S.\",\"Japan\",\"China\",\"Russia\",\"India\",\"India\",\"W. Europe\",\"Russia\",\"India\",\"China\",\"Russia\",\"China\",\"U.S.\",\"W. Europe\",\"Russia\",\"U.S.\",\"Other\",\"U.S.\",\"W. Europe\",\"U.S.\",\"Other\",\"India\",\"India\",\"U.S.\",\"U.S.\",\"China\",\"China\",\"W. Europe\",\"China\",\"W. Europe\",\"India\",\"Japan\",\"India\",\"Japan\",\"W. Europe\",\"U.S.\",\"U.S.\",\"U.S.\",\"Russia\",\"U.S.\",\"W. Europe\",\"Russia\",\"W. Europe\",\"U.S.\",\"China\",\"W. Europe\",\"Other\",\"U.S.\",\"W. Europe\",\"U.S.\",\"Russia\",\"India\",\"India\",\"W. Europe\",\"U.S.\",\"Japan\",\"Russia\",\"W. Europe\",\"W. Europe\",\"Russia\",\"W. Europe\",\"China\",\"Other\",\"U.S.\",\"Russia\",\"W. Europe\",\"India\",\"Japan\",\"China\",\"China\",\"Other\",\"Other\",\"Other\",\"Japan\",\"Japan\",\"U.S.\",\"U.S.\",\"Other\",\"Other\",\"U.S.\",\"India\",\"Other\",\"Japan\",\"Other\",\"Russia\",\"Other\",\"W. Europe\",\"India\",\"U.S.\",\"U.S.\",\"U.S.\",\"W. Europe\",\"W. Europe\",\"Other\",\"U.S.\",\"W. Europe\",\"Russia\",\"India\",\"Japan\",\"Other\",\"Other\",\"W. Europe\",\"Russia\",\"U.S.\",\"W. Europe\",\"W. Europe\",\"U.S.\",\"U.S.\",\"U.S.\",\"India\",\"U.S.\",\"U.S.\",\"China\",\"India\",\"China\",\"W. Europe\",\"U.S.\",\"U.S.\",\"Russia\",\"Russia\",\"Other\",\"Russia\",\"Russia\",\"Other\",\"W. Europe\",\"Russia\",\"Japan\",\"Other\",\"Japan\",\"Russia\",\"W. Europe\",\"U.S.\",\"Other\",\"Russia\",\"U.S.\",\"W. Europe\",\"Russia\",\"Other\",\"U.S.\",\"W. Europe\",\"Russia\",\"U.S.\",\"U.S.\",\"U.S.\",\"Japan\",\"U.S.\",\"U.S.\",\"U.S.\",\"W. Europe\",\"U.S.\",\"U.S.\",\"U.S.\",\"India\",\"Japan\",\"Japan\",\"U.S.\",\"U.S.\",\"U.S.\",\"China\",\"W. Europe\",\"U.S.\",\"U.S.\",\"India\",\"W. Europe\",\"U.S.\",\"Japan\",\"W. Europe\",\"U.S.\",\"W. Europe\",\"W. Europe\",\"W. Europe\",\"Russia\",\"U.S.\",\"China\",\"Japan\",\"China\",\"India\",\"U.S.\",\"Other\",\"Other\",\"China\",\"U.S.\",\"Japan\",\"China\",\"China\",\"Japan\",\"U.S.\",\"W. Europe\",\"Russia\",\"China\",\"Other\",\"W. Europe\",\"W. Europe\",\"U.S.\",\"Japan\",\"U.S.\",\"W. Europe\",\"W. Europe\",\"India\",\"India\",\"Japan\",\"U.S.\",\"W. Europe\",\"Japan\",\"India\",\"U.S.\",\"Other\",\"U.S.\",\"China\",\"India\",\"W. Europe\",\"U.S.\",\"W. Europe\",\"Russia\",\"W. Europe\",\"China\",\"China\",\"Russia\",\"China\",\"Russia\",\"Russia\",\"U.S.\",\"India\",\"Other\",\"Other\",\"Other\",\"U.S.\",\"China\",\"India\",\"India\",\"U.S.\",\"U.S.\",\"India\",\"U.S.\",\"U.S.\",\"Japan\",\"Russia\",\"China\",\"China\",\"India\",\"U.S.\",\"U.S.\",\"W. Europe\",\"Other\",\"Other\",\"Other\",\"U.S.\",\"China\",\"U.S.\",\"Japan\",\"China\",\"W. Europe\",\"W. Europe\",\"W. Europe\",\"Russia\",\"China\",\"China\",\"U.S.\",\"India\",\"China\"], categories=[\"Low Earth Orbit\",\"Low Earth Orbit\",\"Medium Earth Orbit\",\"Medium Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Geosynchronous Orbit\",\"Medium Earth Orbit\",\"Low Earth Orbit\",\"Other Orbit\",\"Low Earth Orbit\",\"Other Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Other Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Other Orbit\",\"Low Earth Orbit\",\"Geosynchronous Orbit\",\"Other Orbit\",\"Other Orbit\",\"Low Earth Orbit\",\"Geosynchronous Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Medium Earth Orbit\",\"Low Earth Orbit\",\"Geosynchronous Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Other Orbit\",\"Geosynchronous Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Geosynchronous Orbit\",\"Low Earth Orbit\",\"Geosynchronous Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Other Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Other Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Medium Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Geosynchronous Orbit\",\"Geosynchronous Orbit\",\"Low Earth Orbit\",\"Medium Earth Orbit\",\"Other Orbit\",\"Other Orbit\",\"Low Earth Orbit\",\"Geosynchronous Orbit\",\"Low Earth Orbit\",\"Other Orbit\",\"Other Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Other Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Geosynchronous Orbit\",\"Geosynchronous Orbit\",\"Geosynchronous Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Medium Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Geosynchronous Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Other Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Medium Earth Orbit\",\"Geosynchronous Orbit\",\"Other Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Geosynchronous Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Other Orbit\",\"Low Earth Orbit\",\"Geosynchronous Orbit\",\"Other Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Geosynchronous Orbit\",\"Medium Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Geosynchronous Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Medium Earth Orbit\",\"Geosynchronous Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Medium Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Other Orbit\",\"Other Orbit\",\"Geosynchronous Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Geosynchronous Orbit\",\"Other Orbit\",\"Medium Earth Orbit\",\"Low Earth Orbit\",\"Geosynchronous Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Geosynchronous Orbit\",\"Geosynchronous Orbit\",\"Low Earth Orbit\",\"Other Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Other Orbit\",\"Geosynchronous Orbit\",\"Geosynchronous Orbit\",\"Geosynchronous Orbit\",\"Other Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Other Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Geosynchronous Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Geosynchronous Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Geosynchronous Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Other Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Geosynchronous Orbit\",\"Geosynchronous Orbit\",\"Other Orbit\",\"Medium Earth Orbit\",\"Low Earth Orbit\",\"Medium Earth Orbit\",\"Low Earth Orbit\",\"Other Orbit\",\"Low Earth Orbit\",\"Geosynchronous Orbit\",\"Low Earth Orbit\",\"Geosynchronous Orbit\",\"Low Earth Orbit\",\"Medium Earth Orbit\",\"Geosynchronous Orbit\",\"Low Earth Orbit\",\"Geosynchronous Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Geosynchronous Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Geosynchronous Orbit\",\"Other Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Geosynchronous Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Geosynchronous Orbit\",\"Low Earth Orbit\",\"Medium Earth Orbit\",\"Other Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Other Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Other Orbit\",\"Low Earth Orbit\",\"Medium Earth Orbit\",\"Medium Earth Orbit\",\"Medium Earth Orbit\",\"Other Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Other Orbit\",\"Low Earth Orbit\",\"Geosynchronous Orbit\",\"Geosynchronous Orbit\",\"Other Orbit\",\"Other Orbit\",\"Low Earth Orbit\",\"Other Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Geosynchronous Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Medium Earth Orbit\",\"Medium Earth Orbit\",\"Other Orbit\",\"Geosynchronous Orbit\",\"Geosynchronous Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Medium Earth Orbit\",\"Low Earth Orbit\",\"Geosynchronous Orbit\",\"Geosynchronous Orbit\",\"Geosynchronous Orbit\",\"Other Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Medium Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Medium Earth Orbit\",\"Geosynchronous Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Other Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Other Orbit\",\"Other Orbit\",\"Geosynchronous Orbit\",\"Medium Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Other Orbit\",\"Other Orbit\",\"Medium Earth Orbit\",\"Low Earth Orbit\",\"Other Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Other Orbit\",\"Geosynchronous Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Medium Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Geosynchronous Orbit\",\"Low Earth Orbit\",\"Other Orbit\",\"Other Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Other Orbit\",\"Low Earth Orbit\",\"Medium Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Other Orbit\",\"Other Orbit\",\"Geosynchronous Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Geosynchronous Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Geosynchronous Orbit\",\"Low Earth Orbit\",\"Other Orbit\",\"Geosynchronous Orbit\",\"Geosynchronous Orbit\",\"Geosynchronous Orbit\",\"Low Earth Orbit\",\"Medium Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Geosynchronous Orbit\",\"Low Earth Orbit\",\"Geosynchronous Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Geosynchronous Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Other Orbit\",\"Low Earth Orbit\",\"Medium Earth Orbit\",\"Geosynchronous Orbit\",\"Low Earth Orbit\",\"Geosynchronous Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Other Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Other Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Geosynchronous Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Geosynchronous Orbit\",\"Geosynchronous Orbit\",\"Low Earth Orbit\",\"Other Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Geosynchronous Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Geosynchronous Orbit\",\"Low Earth Orbit\",\"Other Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Geosynchronous Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Geosynchronous Orbit\",\"Other Orbit\",\"Other Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Medium Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Geosynchronous Orbit\",\"Other Orbit\",\"Geosynchronous Orbit\",\"Other Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Geosynchronous Orbit\",\"Medium Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Geosynchronous Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Medium Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Medium Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Geosynchronous Orbit\",\"Medium Earth Orbit\",\"Medium Earth Orbit\",\"Low Earth Orbit\",\"Geosynchronous Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Other Orbit\",\"Other Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Geosynchronous Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Medium Earth Orbit\",\"Low Earth Orbit\",\"Geosynchronous Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Geosynchronous Orbit\",\"Other Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Geosynchronous Orbit\",\"Medium Earth Orbit\",\"Other Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Geosynchronous Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Geosynchronous Orbit\",\"Low Earth Orbit\",\"Geosynchronous Orbit\",\"Other Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Geosynchronous Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Medium Earth Orbit\",\"Other Orbit\",\"Low Earth Orbit\",\"Geosynchronous Orbit\",\"Geosynchronous Orbit\",\"Other Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Other Orbit\",\"Other Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Medium Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Geosynchronous Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Other Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Geosynchronous Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Other Orbit\",\"Low Earth Orbit\",\"Geosynchronous Orbit\",\"Medium Earth Orbit\",\"Geosynchronous Orbit\",\"Low Earth Orbit\",\"Geosynchronous Orbit\",\"Low Earth Orbit\",\"Other Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Other Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Other Orbit\",\"Geosynchronous Orbit\",\"Other Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Geosynchronous Orbit\",\"Low Earth Orbit\",\"Other Orbit\",\"Other Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Other Orbit\",\"Other Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Medium Earth Orbit\",\"Low Earth Orbit\",\"Geosynchronous Orbit\",\"Medium Earth Orbit\",\"Low Earth Orbit\",\"Geosynchronous Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Other Orbit\",\"Low Earth Orbit\",\"Geosynchronous Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Medium Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Medium Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Other Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Other Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Geosynchronous Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Other Orbit\",\"Geosynchronous Orbit\",\"Medium Earth Orbit\",\"Low Earth Orbit\",\"Geosynchronous Orbit\",\"Geosynchronous Orbit\",\"Medium Earth Orbit\",\"Low Earth Orbit\",\"Other Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Geosynchronous Orbit\",\"Geosynchronous Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Geosynchronous Orbit\",\"Low Earth Orbit\",\"Other Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Medium Earth Orbit\",\"Low Earth Orbit\",\"Other Orbit\",\"Geosynchronous Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Other Orbit\",\"Geosynchronous Orbit\",\"Low Earth Orbit\",\"Other Orbit\",\"Low Earth Orbit\",\"Geosynchronous Orbit\",\"Medium Earth Orbit\",\"Geosynchronous Orbit\",\"Other Orbit\",\"Low Earth Orbit\",\"Geosynchronous Orbit\",\"Low Earth Orbit\",\"Geosynchronous Orbit\",\"Other Orbit\",\"Low Earth Orbit\",\"Geosynchronous Orbit\",\"Geosynchronous Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Geosynchronous Orbit\",\"Low Earth Orbit\",\"Other Orbit\",\"Medium Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Geosynchronous Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Medium Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Medium Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Other Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Other Orbit\",\"Medium Earth Orbit\",\"Low Earth Orbit\",\"Geosynchronous Orbit\",\"Low Earth Orbit\",\"Geosynchronous Orbit\",\"Geosynchronous Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Geosynchronous Orbit\",\"Low Earth Orbit\",\"Geosynchronous Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Other Orbit\",\"Other Orbit\",\"Geosynchronous Orbit\",\"Low Earth Orbit\",\"Medium Earth Orbit\",\"Geosynchronous Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Other Orbit\",\"Geosynchronous Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Geosynchronous Orbit\",\"Low Earth Orbit\",\"Medium Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Other Orbit\",\"Medium Earth Orbit\",\"Low Earth Orbit\",\"Other Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Medium Earth Orbit\",\"Geosynchronous Orbit\",\"Geosynchronous Orbit\",\"Low Earth Orbit\",\"Other Orbit\",\"Geosynchronous Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Geosynchronous Orbit\",\"Geosynchronous Orbit\",\"Low Earth Orbit\",\"Geosynchronous Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Geosynchronous Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Other Orbit\",\"Other Orbit\",\"Low Earth Orbit\",\"Geosynchronous Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Other Orbit\",\"Geosynchronous Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Medium Earth Orbit\",\"Other Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Geosynchronous Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Geosynchronous Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Geosynchronous Orbit\",\"Low Earth Orbit\",\"Geosynchronous Orbit\",\"Other Orbit\",\"Other Orbit\",\"Medium Earth Orbit\",\"Low Earth Orbit\",\"Medium Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Other Orbit\",\"Geosynchronous Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Medium Earth Orbit\",\"Other Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Medium Earth Orbit\",\"Low Earth Orbit\",\"Other Orbit\",\"Geosynchronous Orbit\",\"Geosynchronous Orbit\",\"Low Earth Orbit\",\"Geosynchronous Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Geosynchronous Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Medium Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Other Orbit\",\"Low Earth Orbit\",\"Other Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Other Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Medium Earth Orbit\",\"Geosynchronous Orbit\",\"Geosynchronous Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Geosynchronous Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Other Orbit\",\"Low Earth Orbit\",\"Geosynchronous Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Geosynchronous Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Geosynchronous Orbit\",\"Medium Earth Orbit\",\"Medium Earth Orbit\",\"Low Earth Orbit\",\"Other Orbit\",\"Geosynchronous Orbit\",\"Other Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Geosynchronous Orbit\",\"Geosynchronous Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Geosynchronous Orbit\",\"Geosynchronous Orbit\",\"Geosynchronous Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Other Orbit\",\"Low Earth Orbit\",\"Medium Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Geosynchronous Orbit\",\"Geosynchronous Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Geosynchronous Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Geosynchronous Orbit\",\"Low Earth Orbit\",\"Geosynchronous Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Geosynchronous Orbit\",\"Geosynchronous Orbit\"], categories2=[\"Research\",\"Navigation\",\"Communications\",\"Research\",\"Navigation\",\"Communications\",\"Communications\",\"Communications\",\"Communications\",\"Earth Observation\",\"Communications\",\"Communications\",\"Research\",\"Communications\",\"Communications\",\"Earth Observation\",\"Navigation\",\"Earth Observation\",\"Technology\",\"Research\",\"Research\",\"Communications\",\"Technology\",\"Earth Observation\",\"Earth Observation\",\"Technology\",\"Research\",\"Navigation\",\"Earth Observation\",\"Earth Observation\",\"Navigation\",\"Earth Observation\",\"Communications\",\"Navigation\",\"Communications\",\"Earth Observation\",\"Earth Observation\",\"Earth Observation\",\"Communications\",\"Navigation\",\"Communications\",\"Research\",\"Communications\",\"Communications\",\"Technology\",\"Earth Observation\",\"Communications\",\"Communications\",\"Communications\",\"Research\",\"Technology\",\"Research\",\"Earth Observation\",\"Earth Observation\",\"Earth Observation\",\"Earth Observation\",\"Communications\",\"Communications\",\"Earth Observation\",\"Research\",\"Navigation\",\"Navigation\",\"Communications\",\"Communications\",\"Research\",\"Communications\",\"Earth Observation\",\"Communications\",\"Earth Observation\",\"Communications\",\"Research\",\"Navigation\",\"Technology\",\"Earth Observation\",\"Earth Observation\",\"Research\",\"Research\",\"Technology\",\"Communications\",\"Research\",\"Earth Observation\",\"Earth Observation\",\"Technology\",\"Navigation\",\"Communications\",\"Communications\",\"Research\",\"Research\",\"Communications\",\"Communications\",\"Earth Observation\",\"Communications\",\"Communications\",\"Technology\",\"Earth Observation\",\"Research\",\"Technology\",\"Technology\",\"Earth Observation\",\"Navigation\",\"Earth Observation\",\"Research\",\"Research\",\"Earth Observation\",\"Navigation\",\"Earth Observation\",\"Navigation\",\"Communications\",\"Research\",\"Earth Observation\",\"Research\",\"Technology\",\"Technology\",\"Communications\",\"Communications\",\"Communications\",\"Communications\",\"Navigation\",\"Research\",\"Research\",\"Research\",\"Communications\",\"Earth Observation\",\"Research\",\"Communications\",\"Research\",\"Research\",\"Technology\",\"Research\",\"Communications\",\"Earth Observation\",\"Communications\",\"Earth Observation\",\"Communications\",\"Research\",\"Technology\",\"Technology\",\"Communications\",\"Earth Observation\",\"Navigation\",\"Research\",\"Communications\",\"Earth Observation\",\"Communications\",\"Research\",\"Communications\",\"Research\",\"Technology\",\"Communications\",\"Technology\",\"Navigation\",\"Research\",\"Earth Observation\",\"Earth Observation\",\"Communications\",\"Communications\",\"Research\",\"Navigation\",\"Earth Observation\",\"Research\",\"Earth Observation\",\"Research\",\"Communications\",\"Research\",\"Communications\",\"Communications\",\"Earth Observation\",\"Communications\",\"Earth Observation\",\"Navigation\",\"Communications\",\"Communications\",\"Technology\",\"Technology\",\"Earth Observation\",\"Research\",\"Research\",\"Technology\",\"Research\",\"Technology\",\"Communications\",\"Earth Observation\",\"Communications\",\"Communications\",\"Technology\",\"Communications\",\"Research\",\"Research\",\"Navigation\",\"Earth Observation\",\"Technology\",\"Navigation\",\"Earth Observation\",\"Research\",\"Earth Observation\",\"Navigation\",\"Research\",\"Communications\",\"Earth Observation\",\"Communications\",\"Communications\",\"Technology\",\"Communications\",\"Research\",\"Communications\",\"Communications\",\"Technology\",\"Communications\",\"Research\",\"Earth Observation\",\"Communications\",\"Technology\",\"Communications\",\"Communications\",\"Technology\",\"Communications\",\"Earth Observation\",\"Communications\",\"Technology\",\"Earth Observation\",\"Technology\",\"Research\",\"Research\",\"Earth Observation\",\"Communications\",\"Technology\",\"Earth Observation\",\"Research\",\"Earth Observation\",\"Technology\",\"Communications\",\"Navigation\",\"Communications\",\"Technology\",\"Technology\",\"Technology\",\"Research\",\"Communications\",\"Earth Observation\",\"Earth Observation\",\"Communications\",\"Research\",\"Communications\",\"Research\",\"Research\",\"Earth Observation\",\"Earth Observation\",\"Earth Observation\",\"Earth Observation\",\"Communications\",\"Research\",\"Research\",\"Communications\",\"Research\",\"Research\",\"Communications\",\"Research\",\"Research\",\"Navigation\",\"Navigation\",\"Communications\",\"Communications\",\"Research\",\"Research\",\"Technology\",\"Technology\",\"Communications\",\"Technology\",\"Earth Observation\",\"Navigation\",\"Communications\",\"Technology\",\"Earth Observation\",\"Communications\",\"Technology\",\"Research\",\"Research\",\"Earth Observation\",\"Communications\",\"Earth Observation\",\"Research\",\"Earth Observation\",\"Technology\",\"Technology\",\"Research\",\"Earth Observation\",\"Earth Observation\",\"Communications\",\"Communications\",\"Research\",\"Earth Observation\",\"Research\",\"Communications\",\"Communications\",\"Research\",\"Technology\",\"Communications\",\"Research\",\"Technology\",\"Research\",\"Technology\",\"Navigation\",\"Earth Observation\",\"Technology\",\"Communications\",\"Communications\",\"Earth Observation\",\"Communications\",\"Research\",\"Research\",\"Research\",\"Technology\",\"Communications\",\"Earth Observation\",\"Technology\",\"Research\",\"Communications\",\"Communications\",\"Research\",\"Communications\",\"Communications\",\"Research\",\"Research\",\"Communications\",\"Earth Observation\",\"Technology\",\"Communications\",\"Technology\",\"Earth Observation\",\"Technology\",\"Communications\",\"Navigation\",\"Earth Observation\",\"Technology\",\"Earth Observation\",\"Earth Observation\",\"Research\",\"Communications\",\"Communications\",\"Research\",\"Research\",\"Navigation\",\"Communications\",\"Communications\",\"Earth Observation\",\"Communications\",\"Research\",\"Earth Observation\",\"Earth Observation\",\"Navigation\",\"Research\",\"Technology\",\"Communications\",\"Research\",\"Navigation\",\"Communications\",\"Research\",\"Communications\",\"Communications\",\"Technology\",\"Communications\",\"Research\",\"Research\",\"Technology\",\"Research\",\"Communications\",\"Communications\",\"Research\",\"Research\",\"Communications\",\"Navigation\",\"Research\",\"Research\",\"Research\",\"Communications\",\"Navigation\",\"Research\",\"Communications\",\"Earth Observation\",\"Communications\",\"Communications\",\"Communications\",\"Research\",\"Communications\",\"Earth Observation\",\"Research\",\"Research\",\"Communications\",\"Technology\",\"Technology\",\"Navigation\",\"Technology\",\"Technology\",\"Research\",\"Communications\",\"Earth Observation\",\"Communications\",\"Technology\",\"Communications\",\"Technology\",\"Research\",\"Communications\",\"Research\",\"Technology\",\"Communications\",\"Technology\",\"Research\",\"Navigation\",\"Communications\",\"Communications\",\"Earth Observation\",\"Technology\",\"Earth Observation\",\"Technology\",\"Technology\",\"Navigation\",\"Navigation\",\"Research\",\"Research\",\"Earth Observation\",\"Research\",\"Communications\",\"Communications\",\"Communications\",\"Research\",\"Communications\",\"Earth Observation\",\"Communications\",\"Navigation\",\"Communications\",\"Research\",\"Navigation\",\"Communications\",\"Technology\",\"Communications\",\"Research\",\"Communications\",\"Technology\",\"Research\",\"Communications\",\"Communications\",\"Earth Observation\",\"Earth Observation\",\"Earth Observation\",\"Research\",\"Communications\",\"Communications\",\"Technology\",\"Research\",\"Earth Observation\",\"Research\",\"Earth Observation\",\"Earth Observation\",\"Research\",\"Navigation\",\"Earth Observation\",\"Communications\",\"Earth Observation\",\"Communications\",\"Communications\",\"Earth Observation\",\"Technology\",\"Technology\",\"Communications\",\"Earth Observation\",\"Research\",\"Technology\",\"Communications\",\"Earth Observation\",\"Navigation\",\"Technology\",\"Earth Observation\",\"Earth Observation\",\"Earth Observation\",\"Technology\",\"Navigation\",\"Communications\",\"Communications\",\"Communications\",\"Communications\",\"Communications\",\"Navigation\",\"Navigation\",\"Navigation\",\"Communications\",\"Technology\",\"Navigation\",\"Communications\",\"Navigation\",\"Communications\",\"Communications\",\"Communications\",\"Communications\",\"Technology\",\"Research\",\"Research\",\"Earth Observation\",\"Navigation\",\"Communications\",\"Earth Observation\",\"Research\",\"Earth Observation\",\"Communications\",\"Research\",\"Communications\",\"Earth Observation\",\"Earth Observation\",\"Research\",\"Earth Observation\",\"Research\",\"Communications\",\"Communications\",\"Earth Observation\",\"Research\",\"Earth Observation\",\"Communications\",\"Research\",\"Navigation\",\"Research\",\"Communications\",\"Research\",\"Earth Observation\",\"Communications\",\"Communications\",\"Communications\",\"Earth Observation\",\"Communications\",\"Technology\",\"Communications\",\"Communications\",\"Communications\",\"Research\",\"Research\",\"Earth Observation\",\"Earth Observation\",\"Navigation\",\"Earth Observation\",\"Research\",\"Earth Observation\",\"Navigation\",\"Communications\",\"Communications\",\"Research\",\"Technology\",\"Earth Observation\",\"Earth Observation\",\"Earth Observation\",\"Research\",\"Communications\",\"Technology\",\"Research\",\"Research\",\"Communications\",\"Technology\",\"Research\",\"Earth Observation\",\"Earth Observation\",\"Technology\",\"Communications\",\"Technology\",\"Communications\",\"Research\",\"Navigation\",\"Communications\",\"Earth Observation\",\"Communications\",\"Research\",\"Navigation\",\"Earth Observation\",\"Communications\",\"Earth Observation\",\"Technology\",\"Research\",\"Communications\",\"Research\",\"Communications\",\"Communications\",\"Navigation\",\"Navigation\",\"Earth Observation\",\"Communications\",\"Technology\",\"Research\",\"Technology\",\"Communications\",\"Technology\",\"Earth Observation\",\"Technology\",\"Navigation\",\"Navigation\",\"Earth Observation\",\"Communications\",\"Research\",\"Technology\",\"Earth Observation\",\"Earth Observation\",\"Navigation\",\"Research\",\"Communications\",\"Communications\",\"Communications\",\"Research\",\"Earth Observation\",\"Research\",\"Earth Observation\",\"Research\",\"Research\",\"Research\",\"Communications\",\"Communications\",\"Research\",\"Research\",\"Communications\",\"Communications\",\"Communications\",\"Research\",\"Communications\",\"Research\",\"Navigation\",\"Communications\",\"Communications\",\"Research\",\"Earth Observation\",\"Research\",\"Technology\",\"Communications\",\"Research\",\"Earth Observation\",\"Earth Observation\",\"Communications\",\"Earth Observation\",\"Earth Observation\",\"Navigation\",\"Earth Observation\",\"Earth Observation\",\"Navigation\",\"Earth Observation\",\"Earth Observation\",\"Research\",\"Communications\",\"Technology\",\"Navigation\",\"Communications\",\"Earth Observation\",\"Navigation\",\"Communications\",\"Earth Observation\",\"Earth Observation\",\"Earth Observation\",\"Communications\",\"Research\",\"Earth Observation\",\"Research\",\"Navigation\",\"Communications\",\"Research\",\"Communications\",\"Technology\",\"Research\",\"Communications\",\"Research\",\"Communications\",\"Research\",\"Earth Observation\",\"Research\",\"Communications\",\"Earth Observation\",\"Technology\",\"Earth Observation\",\"Navigation\",\"Earth Observation\",\"Research\",\"Communications\",\"Technology\",\"Technology\",\"Communications\",\"Navigation\",\"Earth Observation\",\"Research\",\"Research\",\"Communications\",\"Technology\",\"Navigation\",\"Earth Observation\",\"Earth Observation\",\"Earth Observation\",\"Research\",\"Communications\",\"Navigation\",\"Communications\",\"Earth Observation\",\"Research\",\"Research\",\"Earth Observation\",\"Communications\",\"Communications\",\"Technology\",\"Technology\",\"Navigation\",\"Communications\",\"Earth Observation\",\"Communications\",\"Communications\",\"Earth Observation\",\"Earth Observation\",\"Communications\",\"Navigation\",\"Research\",\"Earth Observation\",\"Navigation\",\"Communications\",\"Communications\",\"Research\",\"Research\",\"Research\",\"Earth Observation\",\"Communications\",\"Communications\",\"Earth Observation\",\"Earth Observation\",\"Research\",\"Communications\",\"Navigation\",\"Earth Observation\",\"Research\",\"Earth Observation\",\"Communications\",\"Technology\",\"Research\",\"Communications\",\"Communications\",\"Communications\",\"Technology\",\"Research\",\"Communications\",\"Navigation\",\"Earth Observation\",\"Earth Observation\",\"Communications\",\"Communications\",\"Communications\",\"Communications\",\"Communications\",\"Navigation\",\"Communications\",\"Research\",\"Research\",\"Communications\",\"Technology\",\"Research\",\"Research\",\"Communications\",\"Earth Observation\",\"Communications\",\"Earth Observation\",\"Earth Observation\",\"Communications\",\"Earth Observation\",\"Technology\",\"Earth Observation\",\"Technology\",\"Communications\",\"Earth Observation\",\"Communications\",\"Earth Observation\",\"Research\",\"Navigation\",\"Research\",\"Earth Observation\",\"Research\",\"Communications\",\"Communications\",\"Earth Observation\",\"Technology\",\"Earth Observation\",\"Earth Observation\",\"Communications\",\"Communications\",\"Research\",\"Earth Observation\",\"Earth Observation\",\"Navigation\",\"Communications\",\"Earth Observation\",\"Communications\",\"Research\",\"Research\",\"Communications\",\"Technology\",\"Technology\",\"Earth Observation\",\"Communications\",\"Communications\",\"Research\",\"Communications\",\"Earth Observation\",\"Earth Observation\",\"Earth Observation\",\"Technology\",\"Communications\",\"Research\",\"Earth Observation\",\"Earth Observation\",\"Research\",\"Navigation\",\"Communications\",\"Earth Observation\",\"Technology\",\"Technology\",\"Research\",\"Earth Observation\",\"Technology\",\"Earth Observation\",\"Earth Observation\",\"Communications\",\"Technology\",\"Communications\",\"Communications\",\"Research\",\"Earth Observation\",\"Research\",\"Research\",\"Navigation\",\"Communications\",\"Research\",\"Communications\",\"Navigation\",\"Research\",\"Navigation\",\"Research\",\"Earth Observation\",\"Communications\",\"Research\",\"Research\",\"Communications\",\"Earth Observation\",\"Communications\",\"Communications\",\"Communications\",\"Navigation\",\"Navigation\",\"Communications\",\"Earth Observation\",\"Research\",\"Technology\",\"Earth Observation\",\"Communications\",\"Communications\",\"Communications\",\"Research\",\"Earth Observation\",\"Navigation\",\"Earth Observation\",\"Earth Observation\",\"Research\",\"Research\",\"Earth Observation\",\"Communications\",\"Communications\",\"Research\",\"Communications\",\"Communications\",\"Research\",\"Communications\",\"Technology\",\"Research\",\"Research\",\"Technology\",\"Technology\",\"Earth Observation\",\"Research\",\"Research\",\"Earth Observation\",\"Research\",\"Technology\",\"Communications\",\"Research\",\"Earth Observation\",\"Earth Observation\",\"Communications\",\"Research\",\"Communications\",\"Communications\",\"Communications\",\"Earth Observation\",\"Communications\",\"Technology\",\"Research\",\"Research\",\"Technology\",\"Research\",\"Earth Observation\",\"Earth Observation\",\"Navigation\",\"Navigation\",\"Communications\",\"Communications\",\"Earth Observation\",\"Communications\",\"Earth Observation\",\"Navigation\",\"Earth Observation\",\"Communications\",\"Technology\",\"Technology\",\"Earth Observation\",\"Communications\",\"Earth Observation\",\"Research\",\"Earth Observation\",\"Communications\",\"Research\",\"Communications\",\"Communications\",\"Technology\",\"Technology\",\"Navigation\",\"Communications\",\"Navigation\",\"Technology\",\"Earth Observation\",\"Communications\",\"Research\",\"Communications\",\"Communications\",\"Communications\",\"Navigation\",\"Earth Observation\",\"Technology\",\"Navigation\",\"Earth Observation\",\"Research\",\"Research\",\"Earth Observation\",\"Earth Observation\",\"Technology\",\"Research\",\"Communications\",\"Communications\",\"Navigation\",\"Earth Observation\",\"Technology\",\"Communications\",\"Research\",\"Research\",\"Research\",\"Technology\",\"Earth Observation\",\"Earth Observation\",\"Research\"], symbols=[\"star\",\"circle\",\"triangle\",\"circle\",\"circle\",\"triangle\",\"circle\",\"star\",\"star\",\"triangle\",\"star\",\"circle\",\"circle\",\"circle\",\"star\",\"star\",\"star\",\"circle\",\"circle\",\"circle\",\"circle\",\"star\",\"star\",\"circle\",\"triangle\",\"circle\",\"circle\",\"star\",\"star\",\"circle\",\"triangle\",\"diamond\",\"star\",\"circle\",\"triangle\",\"star\",\"diamond\",\"diamond\",\"diamond\",\"circle\",\"star\",\"star\",\"triangle\",\"star\",\"circle\",\"circle\",\"star\",\"diamond\",\"circle\",\"star\",\"star\",\"star\",\"circle\",\"diamond\",\"triangle\",\"diamond\",\"circle\",\"star\",\"circle\",\"star\",\"star\",\"diamond\",\"circle\",\"circle\",\"circle\",\"circle\",\"circle\",\"circle\",\"circle\",\"star\",\"circle\",\"circle\",\"diamond\",\"circle\",\"circle\",\"circle\",\"star\",\"circle\",\"circle\",\"star\",\"triangle\",\"star\",\"star\",\"star\",\"triangle\",\"diamond\",\"diamond\",\"circle\",\"star\",\"circle\",\"circle\",\"triangle\",\"circle\",\"triangle\",\"star\",\"diamond\",\"triangle\",\"star\",\"star\",\"star\",\"star\",\"star\",\"star\",\"circle\",\"circle\",\"star\",\"diamond\",\"star\",\"circle\",\"star\",\"circle\",\"star\",\"triangle\",\"diamond\",\"circle\",\"circle\",\"circle\",\"diamond\",\"star\",\"diamond\",\"triangle\",\"circle\",\"circle\",\"circle\",\"star\",\"circle\",\"triangle\",\"circle\",\"circle\",\"diamond\",\"star\",\"circle\",\"circle\",\"circle\",\"circle\",\"diamond\",\"star\",\"circle\",\"circle\",\"circle\",\"star\",\"star\",\"circle\",\"triangle\",\"circle\",\"star\",\"circle\",\"circle\",\"star\",\"diamond\",\"star\",\"star\",\"circle\",\"circle\",\"circle\",\"circle\",\"circle\",\"triangle\",\"circle\",\"circle\",\"circle\",\"star\",\"diamond\",\"star\",\"star\",\"star\",\"star\",\"circle\",\"star\",\"circle\",\"diamond\",\"diamond\",\"diamond\",\"star\",\"diamond\",\"star\",\"circle\",\"diamond\",\"circle\",\"diamond\",\"circle\",\"circle\",\"star\",\"star\",\"circle\",\"circle\",\"star\",\"circle\",\"circle\",\"star\",\"circle\",\"star\",\"diamond\",\"circle\",\"circle\",\"circle\",\"diamond\",\"circle\",\"circle\",\"star\",\"circle\",\"circle\",\"circle\",\"circle\",\"circle\",\"circle\",\"circle\",\"circle\",\"circle\",\"star\",\"circle\",\"circle\",\"diamond\",\"star\",\"star\",\"circle\",\"star\",\"star\",\"circle\",\"star\",\"circle\",\"circle\",\"circle\",\"diamond\",\"circle\",\"triangle\",\"diamond\",\"circle\",\"circle\",\"circle\",\"circle\",\"circle\",\"diamond\",\"circle\",\"circle\",\"diamond\",\"circle\",\"triangle\",\"circle\",\"circle\",\"star\",\"circle\",\"circle\",\"diamond\",\"circle\",\"star\",\"circle\",\"star\",\"triangle\",\"triangle\",\"circle\",\"star\",\"circle\",\"circle\",\"circle\",\"circle\",\"circle\",\"star\",\"circle\",\"circle\",\"circle\",\"star\",\"circle\",\"triangle\",\"star\",\"circle\",\"diamond\",\"circle\",\"star\",\"star\",\"circle\",\"triangle\",\"star\",\"triangle\",\"diamond\",\"diamond\",\"circle\",\"circle\",\"star\",\"circle\",\"circle\",\"circle\",\"diamond\",\"circle\",\"triangle\",\"circle\",\"star\",\"circle\",\"circle\",\"star\",\"star\",\"triangle\",\"triangle\",\"circle\",\"star\",\"circle\",\"diamond\",\"triangle\",\"star\",\"circle\",\"triangle\",\"diamond\",\"star\",\"circle\",\"star\",\"circle\",\"star\",\"triangle\",\"diamond\",\"star\",\"star\",\"circle\",\"star\",\"diamond\",\"star\",\"circle\",\"star\",\"triangle\",\"circle\",\"circle\",\"star\",\"circle\",\"star\",\"star\",\"star\",\"circle\",\"circle\",\"circle\",\"circle\",\"star\",\"circle\",\"star\",\"circle\",\"star\",\"triangle\",\"diamond\",\"triangle\",\"circle\",\"star\",\"diamond\",\"circle\",\"circle\",\"star\",\"circle\",\"triangle\",\"diamond\",\"triangle\",\"circle\",\"star\",\"circle\",\"diamond\",\"triangle\",\"circle\",\"star\",\"star\",\"star\",\"circle\",\"diamond\",\"circle\",\"circle\",\"diamond\",\"triangle\",\"circle\",\"star\",\"star\",\"diamond\",\"circle\",\"circle\",\"diamond\",\"star\",\"circle\",\"diamond\",\"circle\",\"star\",\"diamond\",\"triangle\",\"triangle\",\"star\",\"star\",\"star\",\"triangle\",\"circle\",\"circle\",\"circle\",\"star\",\"triangle\",\"star\",\"star\",\"circle\",\"circle\",\"star\",\"circle\",\"triangle\",\"diamond\",\"star\",\"diamond\",\"star\",\"circle\",\"diamond\",\"circle\",\"circle\",\"star\",\"circle\",\"star\",\"circle\",\"star\",\"star\",\"star\",\"star\",\"circle\",\"star\",\"circle\",\"diamond\",\"circle\",\"star\",\"star\",\"star\",\"circle\",\"diamond\",\"triangle\",\"triangle\",\"diamond\",\"diamond\",\"circle\",\"triangle\",\"star\",\"triangle\",\"star\",\"circle\",\"circle\",\"star\",\"circle\",\"circle\",\"circle\",\"star\",\"circle\",\"star\",\"circle\",\"circle\",\"diamond\",\"circle\",\"diamond\",\"star\",\"circle\",\"circle\",\"star\",\"diamond\",\"star\",\"diamond\",\"circle\",\"star\",\"circle\",\"triangle\",\"circle\",\"diamond\",\"circle\",\"circle\",\"circle\",\"star\",\"star\",\"star\",\"circle\",\"circle\",\"circle\",\"circle\",\"circle\",\"diamond\",\"star\",\"triangle\",\"circle\",\"circle\",\"circle\",\"circle\",\"triangle\",\"circle\",\"circle\",\"circle\",\"circle\",\"diamond\",\"star\",\"circle\",\"circle\",\"diamond\",\"circle\",\"circle\",\"circle\",\"diamond\",\"triangle\",\"star\",\"diamond\",\"circle\",\"circle\",\"star\",\"diamond\",\"circle\",\"star\",\"triangle\",\"triangle\",\"circle\",\"circle\",\"triangle\",\"star\",\"circle\",\"circle\",\"circle\",\"triangle\",\"circle\",\"star\",\"circle\",\"star\",\"circle\",\"circle\",\"star\",\"diamond\",\"triangle\",\"star\",\"diamond\",\"star\",\"triangle\",\"circle\",\"triangle\",\"star\",\"star\",\"diamond\",\"circle\",\"star\",\"star\",\"diamond\",\"star\",\"triangle\",\"star\",\"diamond\",\"star\",\"circle\",\"star\",\"diamond\",\"star\",\"circle\",\"circle\",\"circle\",\"circle\",\"star\",\"triangle\",\"diamond\",\"star\",\"circle\",\"diamond\",\"circle\",\"star\",\"diamond\",\"star\",\"circle\",\"circle\",\"star\",\"circle\",\"circle\",\"circle\",\"diamond\",\"circle\",\"circle\",\"diamond\",\"circle\",\"circle\",\"diamond\",\"circle\",\"star\",\"star\",\"star\",\"circle\",\"star\",\"circle\",\"diamond\",\"star\",\"circle\",\"triangle\",\"circle\",\"circle\",\"star\",\"circle\",\"circle\",\"star\",\"star\",\"circle\",\"diamond\",\"diamond\",\"circle\",\"star\",\"circle\",\"star\",\"circle\",\"triangle\",\"circle\",\"star\",\"star\",\"diamond\",\"star\",\"star\",\"star\",\"star\",\"diamond\",\"star\",\"triangle\",\"star\",\"star\",\"circle\",\"star\",\"star\",\"star\",\"triangle\",\"star\",\"circle\",\"star\",\"circle\",\"diamond\",\"star\",\"circle\",\"star\",\"diamond\",\"circle\",\"diamond\",\"triangle\",\"circle\",\"circle\",\"star\",\"star\",\"circle\",\"circle\",\"circle\",\"circle\",\"star\",\"star\",\"circle\",\"circle\",\"circle\",\"circle\",\"circle\",\"star\",\"diamond\",\"circle\",\"star\",\"triangle\",\"star\",\"star\",\"circle\",\"circle\",\"circle\",\"circle\",\"circle\",\"circle\",\"circle\",\"triangle\",\"circle\",\"star\",\"star\",\"star\",\"star\",\"circle\",\"triangle\",\"star\",\"triangle\",\"triangle\",\"triangle\",\"circle\",\"circle\",\"circle\",\"triangle\",\"circle\",\"circle\",\"star\",\"star\",\"circle\",\"star\",\"circle\",\"star\",\"star\",\"star\",\"star\",\"circle\",\"star\",\"diamond\",\"diamond\",\"diamond\",\"diamond\",\"circle\",\"circle\",\"star\",\"diamond\",\"circle\",\"triangle\",\"star\",\"triangle\",\"triangle\",\"triangle\",\"circle\",\"triangle\",\"circle\",\"star\",\"star\",\"circle\",\"star\",\"circle\",\"star\",\"star\",\"diamond\",\"triangle\",\"star\",\"star\",\"diamond\",\"circle\",\"circle\",\"circle\",\"star\",\"circle\",\"diamond\",\"star\",\"diamond\",\"circle\",\"star\",\"circle\",\"triangle\",\"triangle\",\"star\",\"circle\",\"triangle\",\"star\",\"circle\",\"circle\",\"star\",\"triangle\",\"star\",\"circle\",\"diamond\",\"diamond\",\"triangle\",\"circle\",\"circle\",\"diamond\",\"circle\",\"circle\",\"circle\",\"circle\",\"circle\",\"circle\",\"circle\",\"circle\",\"star\",\"triangle\",\"triangle\",\"circle\",\"circle\",\"star\",\"diamond\",\"diamond\",\"triangle\",\"star\",\"circle\",\"circle\",\"circle\",\"circle\",\"diamond\",\"circle\",\"diamond\",\"circle\",\"circle\",\"circle\",\"star\",\"star\",\"star\",\"circle\",\"triangle\",\"star\",\"circle\",\"diamond\",\"circle\",\"star\",\"star\",\"star\",\"star\",\"triangle\",\"circle\",\"circle\",\"triangle\",\"star\",\"circle\",\"star\",\"star\",\"diamond\",\"star\",\"triangle\",\"circle\",\"circle\",\"circle\",\"diamond\",\"circle\",\"star\",\"circle\",\"circle\",\"diamond\",\"star\",\"star\",\"circle\",\"star\",\"circle\",\"star\",\"star\",\"star\",\"star\",\"star\",\"star\",\"diamond\",\"circle\",\"diamond\",\"triangle\",\"star\",\"star\",\"star\",\"star\",\"star\",\"circle\",\"circle\",\"circle\",\"star\",\"circle\",\"circle\",\"circle\",\"circle\",\"star\",\"triangle\",\"circle\",\"circle\",\"circle\",\"circle\",\"diamond\",\"triangle\",\"circle\",\"star\",\"star\",\"circle\",\"diamond\",\"star\",\"circle\",\"diamond\",\"circle\",\"diamond\",\"circle\",\"diamond\",\"star\",\"circle\",\"diamond\",\"circle\",\"star\",\"circle\",\"circle\",\"diamond\",\"star\",\"circle\",\"diamond\",\"triangle\",\"circle\",\"diamond\",\"circle\",\"triangle\",\"diamond\",\"diamond\",\"diamond\",\"star\",\"circle\",\"star\",\"circle\",\"triangle\",\"circle\",\"diamond\",\"diamond\",\"circle\",\"star\",\"triangle\",\"circle\",\"circle\",\"diamond\",\"diamond\",\"star\",\"star\",\"circle\",\"triangle\",\"circle\",\"star\",\"star\",\"star\",\"triangle\",\"star\",\"circle\",\"circle\",\"star\",\"triangle\",\"diamond\",\"star\",\"circle\",\"star\",\"star\",\"circle\",\"star\",\"circle\",\"diamond\",\"circle\",\"triangle\",\"star\",\"circle\",\"star\",\"diamond\",\"star\",\"triangle\",\"circle\",\"triangle\",\"triangle\",\"circle\",\"star\",\"triangle\",\"triangle\",\"star\",\"triangle\",\"star\",\"star\",\"diamond\",\"triangle\",\"circle\",\"circle\",\"circle\",\"circle\",\"circle\",\"circle\",\"triangle\",\"star\",\"diamond\",\"diamond\",\"triangle\",\"circle\",\"circle\",\"diamond\",\"circle\",\"circle\",\"circle\",\"circle\",\"circle\",\"diamond\",\"circle\",\"diamond\",\"star\",\"circle\",\"circle\",\"star\"], values=[476.7,424.1,57.9,70.7,240.9,59.7,263.9,131.2,266.8,8.9,3437.1,223.1,201.1,704.5,83.7,785.2,176.3,80.8,748.4,452.2,54.3,313.5,542.6,68.8,1576.3,868.5,719.9,109.0,4104.8,115.4,16.1,2307.1,272.9,157.1,49.5,464.7,2023.7,884.1,5184.2,107.1,251.4,897.3,43.6,4463.4,660.9,306.5,1385.7,1081.9,3782.6,632.6,489.6,301.8,44.2,3276.6,48.2,4762.4,248.5,138.7,210.0,502.8,157.4,1656.6,3139.5,4626.1,646.1,131.3,138.9,858.3,220.7,4896.7,112.9,542.8,4384.4,196.1,345.8,225.8,643.1,385.3,161.8,736.2,23.8,284.6,283.8,3244.0,5922.3,1427.2,3455.9,173.1,180.7,195.5,629.1,30.6,182.0,10.5,154.7,3421.7,20.2,437.3,297.0,465.5,355.1,280.1,483.9,207.7,117.2,528.0,1169.8,103.4,434.8,660.0,1219.3,650.7,23.2,2874.4,3493.8,221.2,121.4,2932.9,230.1,5070.9,27.6,142.3,531.8,2549.6,507.3,750.6,19.6,630.9,3426.6,4566.7,296.5,106.3,127.6,5384.6,703.2,4412.9,313.2,162.1,128.3,470.4,844.9,3287.5,877.3,10.9,629.2,1581.4,690.3,394.3,143.3,3215.9,691.9,749.6,817.7,316.8,200.6,174.9,3930.8,1692.8,666.2,346.6,163.2,90.7,5179.8,303.2,5889.0,2544.7,5622.2,154.0,79.0,890.0,5146.0,1065.0,3073.6,550.7,4260.9,264.7,170.6,936.1,165.8,5026.5,245.8,874.7,205.2,193.3,2466.0,307.1,528.0,3093.6,215.1,196.2,276.1,227.1,3983.2,260.4,482.0,647.7,3153.9,2924.8,5008.2,613.3,377.5,835.9,334.0,826.5,768.2,294.0,2907.2,210.3,5748.1,58.1,507.7,5317.3,1715.1,3653.3,595.8,96.5,1401.9,271.1,776.2,575.8,198.8,2555.5,653.4,2580.1,288.8,24.0,3980.9,529.2,263.2,2881.5,214.3,847.4,4956.7,482.3,250.7,4179.1,536.1,38.0,327.9,338.8,176.8,688.6,647.2,4221.8,570.1,138.2,237.8,284.4,44.5,45.4,138.5,255.6,4418.5,3683.3,354.6,397.8,889.6,895.2,558.6,279.3,129.3,181.9,645.1,5.6,425.6,609.1,1907.3,4487.2,615.5,351.1,58.1,4.9,467.7,4679.6,1798.4,1279.1,476.3,343.4,213.9,4021.7,1253.5,4293.1,1002.7,810.3,6.7,450.1,180.4,155.5,311.9,762.5,688.3,39.2,50.1,319.9,59.2,624.5,3779.6,2979.6,137.9,275.7,30.1,4201.0,487.2,645.3,218.0,297.9,238.9,21.8,4345.3,336.0,353.1,2446.0,265.9,1504.5,607.6,717.1,145.0,39.8,356.0,531.0,124.3,160.0,156.0,289.4,629.7,825.2,753.6,3056.1,377.1,342.6,195.0,533.9,664.7,419.8,59.4,3097.4,4.1,795.2,853.2,3740.5,199.0,488.9,84.0,216.4,29.8,2562.3,2.4,53.1,576.6,544.7,4680.7,19.5,631.1,4628.1,117.3,92.1,5714.1,1466.5,280.2,296.0,5980.8,39.0,199.9,4545.6,5917.1,3699.5,192.0,837.3,2266.4,92.1,321.7,3138.6,412.4,5936.9,1050.6,2540.8,12.4,185.0,4606.6,176.6,51.3,230.0,720.5,221.6,5597.8,53.3,4543.9,254.5,567.6,468.4,695.1,175.1,17.2,2975.6,190.5,1131.5,115.2,307.6,4051.6,324.4,447.6,2995.7,418.4,560.0,2343.8,5905.3,781.6,77.7,165.4,180.0,2759.4,742.4,4727.4,432.4,4922.7,790.6,306.5,167.5,3402.7,40.8,4273.1,2269.0,1784.7,1349.6,44.8,369.3,15.2,171.4,657.0,108.1,562.8,2148.3,233.8,5326.4,164.7,136.8,97.0,44.8,5143.7,4678.3,319.3,4168.3,5993.7,531.8,580.7,287.0,1674.2,591.6,1582.8,309.0,864.3,668.4,9.2,137.1,1505.1,4556.1,504.0,449.7,166.3,3728.7,347.3,168.3,370.1,107.2,883.7,72.0,4809.4,103.2,58.5,187.4,3301.1,249.4,878.2,50.0,393.1,632.0,126.6,233.2,2964.0,87.2,99.9,70.0,3809.6,119.6,98.8,3850.2,4165.9,19.3,129.1,1105.2,4471.4,209.9,578.9,4923.8,44.5,339.2,5579.7,33.5,107.4,1726.4,12.8,2975.5,335.2,434.7,274.3,36.1,1515.2,357.4,84.5,671.2,268.5,312.5,538.0,3162.9,10.7,5298.1,2575.6,106.2,8.3,203.6,49.9,604.6,281.8,3211.6,298.1,166.2,178.4,4486.4,4636.0,45.9,243.3,1403.6,647.3,611.1,103.3,3493.5,210.2,49.7,793.0,729.8,152.4,95.7,1866.7,5153.0,369.6,615.5,5095.3,4201.6,542.9,3676.8,140.2,3777.4,129.2,446.3,112.7,858.0,572.8,3999.5,198.2,828.8,1792.0,747.4,247.3,1783.2,4355.7,850.0,449.7,728.8,786.2,4848.6,156.6,4864.2,573.0,276.3,8.2,99.0,243.3,59.3,80.0,394.4,139.2,265.9,860.6,3361.8,2565.0,539.4,252.5,1786.8,623.2,131.5,37.1,205.6,1904.1,878.2,2968.6,331.2,569.6,351.0,254.5,1579.8,188.5,51.5,515.4,333.4,177.1,588.7,328.1,575.4,26.7,289.7,182.2,667.0,452.5,2939.2,245.0,247.5,104.7,3391.3,502.8,4578.0,26.6,388.4,376.5,541.5,487.4,307.5,767.2,671.2,473.8,133.7,525.0,1616.9,287.3,301.1,534.2,622.8,3798.7,1130.6,296.5,1672.5,3460.5,366.4,223.9,695.5,77.0,206.1,625.5,116.6,178.1,5816.3,3265.6,873.4,167.0,248.7,240.8,5426.9,107.4,29.6,490.1,39.6,60.0,28.0,317.5,92.0,730.8,27.5,248.5,482.0,2045.5,797.6,194.0,375.5,3487.3,256.4,682.4,93.5,1575.1,630.7,2234.3,4052.5,5069.6,4997.2,3385.5,4206.4,892.2,514.8,1516.2,3455.8,29.9,168.0,56.4,47.8,40.7,6152.0,23.4,284.9,185.8,712.9,383.6,181.1,638.3,1490.8,96.7,2527.4,3.2,310.8,41.3,4562.7,851.2,793.9,350.5,156.5,687.4,2680.1,821.6,4370.1,218.5,175.1,505.4,52.5,59.2,805.0,89.0,47.7,2600.7,114.5,1611.6,2915.0,51.4,223.7,179.5,3457.1,4507.2,1.9,193.6,496.2,2559.8,2161.1,286.2,1336.9,123.4,202.0,182.9,91.1,318.7,689.4,49.2,47.9,682.2,773.1,4820.2,2092.0,2285.5,3924.6,821.4,741.7,172.7,5262.6,119.4,5162.2,3237.6,1004.5,82.0,219.5,294.6,251.4,438.9,887.4,568.7,38.8,848.0,168.8,2089.8,488.5,375.2,5637.1,1287.6,98.6,9.1,5103.0,661.4,34.7,3298.2,1660.8,271.0,4946.3,3893.8,275.8,23.1,407.3,768.1,4590.9,2972.1,59.2,218.2,435.1,374.3,5044.9,819.0,4115.6,292.7,460.4,193.8,2453.8,694.4,127.2,533.9,848.2,888.2,3487.0,204.0,4198.3,32.5,613.7,271.5,5008.0,90.2,163.2,446.8,458.8,462.5,177.3,844.7,280.0,72.9,5593.0,429.7,11.2,410.8,3493.8,483.2,1836.9,1834.6,26.0,244.4,318.1,544.8,257.1,4033.8,533.9,4966.7,1405.1,527.3,3038.5,866.7,1942.8,270.1,167.7,3633.4,88.6,276.4,234.4,873.3,1557.1,6057.9,847.1,2272.4,53.3,111.6,2881.7,314.8,21.2,2179.3,3458.1,3076.2,393.9,754.0,786.7,483.6,27.7,465.4,1374.8,2930.9,673.5,287.5,7.8,489.7,591.8,2071.3,4511.5,140.5,141.2,154.2,37.6,309.6,783.4,4814.0,1784.1,31.0,387.8,2863.1,99.2,332.2,49.1,4819.3,2087.0,132.6,201.3,1405.8,695.1,512.0,4924.8,3323.1,617.2,24.4,67.8,2687.1,881.7,2431.7,228.1,33.2,5095.8,5267.5,48.9,828.9,353.8,54.3,51.6,254.2,49.9,130.7,1282.9,5432.8,4642.0,183.8,312.2,314.9,179.3,498.4,759.0,14.9,57.0,3687.9,3190.4,3464.0,1262.7,219.9,5197.2,90.5,1807.3,473.2,589.0,5363.6,1765.4,4234.6,1610.8,375.9,58.0,5603.8,3645.4], color_values=[2012,1984,2016,1990,1993,1980,2000,1978,1992,2020,1987,1992,2013,1996,1998,1986,2015,1981,2013,2016,1992,1980,1983,2013,2009,1996,2006,2015,2003,1976,1975,2003,2023,2006,1982,1995,2013,1983,2015,1987,1986,1985,1983,2007,2016,2025,1996,1977,2021,2023,2010,1984,1999,2006,2000,2005,1976,2006,2008,2015,2022,2024,2002,1976,2015,1977,1986,2004,1998,2000,2020,1996,2000,1988,2015,2000,2023,1978,2009,1979,1985,2000,2020,2018,1979,1981,1982,1998,2024,2007,2011,2016,2014,1990,1979,2022,2010,1995,2024,2023,1982,2008,1980,2011,2004,1976,1980,2016,1997,2020,1999,1982,1985,2021,2012,2013,2023,2000,1978,2025,1990,2008,1986,2020,2007,1988,2000,1981,1999,2021,1996,1994,2014,2012,1996,1999,2025,2016,2020,1984,2011,1996,1983,2022,1998,1977,2022,1977,2011,2006,2023,1982,1995,2025,2000,1990,2011,2025,1976,2012,2024,1995,1987,1995,2003,2021,2012,2017,2013,1987,1977,1979,2007,2024,2025,1987,1976,1982,1994,2006,2013,1977,1977,1979,2010,2014,2012,1978,1988,2006,1976,1981,1982,2008,1983,1991,2003,1988,2022,2024,1978,1987,2002,2000,1994,2011,1998,2014,1977,2013,2000,2019,2000,2011,1998,1981,2014,2000,1988,1992,1975,1991,1982,1976,2016,2019,1991,1992,1978,1983,2010,1990,2009,1975,2022,2005,1989,2003,2022,1984,1995,1981,1986,2008,1997,2022,2013,1977,1995,2006,1981,1997,1979,1978,1981,2014,1988,1999,2019,1991,2020,2014,1990,2016,1979,1975,2001,2004,1997,2004,2025,2004,1984,2025,2004,2001,1982,1990,2004,1986,1984,2000,2001,1988,2015,1997,1999,2005,1975,1988,1978,1998,1996,2006,2018,1979,1982,2010,1988,2001,2019,1995,1999,2000,1991,1979,2023,1985,1981,1985,2025,1976,1979,2009,2012,1989,2021,1979,1997,2018,2008,1987,1994,1984,2014,1981,2019,2025,2024,2016,2025,2011,2018,1977,1999,1993,1977,1989,1987,1998,1997,2011,1994,1993,1977,1978,2002,1994,1995,1991,1985,2010,1997,2017,2006,1992,2007,1995,1994,1986,2017,1979,2019,2002,2015,1989,2019,1976,1992,2023,1979,1988,2018,2012,1984,2016,1991,1983,2014,1999,2006,2021,2020,1976,2004,1979,2002,2011,1989,1978,2022,1976,2011,2006,2008,1994,2012,2014,1991,2014,1995,2010,2002,1997,2020,1976,1979,1984,1998,1986,2023,1998,1989,2008,2004,1987,1993,1999,2009,1988,1975,2010,1985,1985,1995,2017,1992,2017,2006,1998,1988,2021,2002,1984,2020,1993,2009,1994,2000,2020,1981,1982,2011,1991,1992,1976,2010,2005,2004,2022,1983,2011,2018,2016,1997,2021,1995,1991,1998,1992,1982,2002,1988,2022,1993,2016,1986,1985,2007,2008,2019,2001,2013,2025,1998,1984,1986,2015,2005,2015,2022,2001,1982,2001,2004,2001,2003,2003,1996,1992,2002,2009,2005,1988,1983,2004,2011,2002,2025,1975,1999,2005,2008,2003,1977,2001,1978,2004,1993,1976,2010,2019,2008,2010,2011,2002,1980,2003,2003,2008,1989,1993,1977,2003,2023,1988,2016,1977,2018,1999,2010,2001,1986,1978,2004,1998,1977,1995,2011,1996,2019,1997,1994,2024,1995,1991,2010,1979,2025,2003,2021,1992,1975,1978,1976,1995,2007,1990,1998,1980,1997,1990,2003,1980,2004,1988,1982,2013,2018,1988,1992,1992,2010,2022,2014,2002,1992,1997,2010,1980,2014,1987,2012,1990,1998,1990,2016,1983,2010,1993,2019,1986,2007,2014,2021,1985,2025,2010,2022,2020,2010,1998,2025,2007,1989,2015,1991,1999,1989,1992,1975,1990,1982,1989,1989,2005,1977,1995,2019,1980,2021,2024,2024,2000,1998,2005,2022,2017,1989,2024,2016,2024,2015,2019,2023,2008,1998,1979,2016,1999,2024,1979,1998,2015,2010,2022,1997,1982,2000,1988,2012,2006,2017,2008,1980,2014,1976,2009,1998,2008,2009,2018,1990,1977,1981,2018,2020,2007,2010,1977,1987,1986,2006,2017,2022,1986,1998,2019,2023,1999,1982,2007,1983,2014,2020,1995,1992,1978,2003,2012,2007,1997,1999,1990,2024,1996,2015,2015,2021,1990,1998,2012,1983,1978,2016,1986,1987,1988,2000,1992,1994,2021,2024,1993,2021,2010,1989,2018,2003,2002,1977,2021,1983,2008,1981,2018,2017,2006,2009,1980,1996,1983,1984,1997,1982,1980,2023,2001,2001,1976,1984,1986,1975,2013,2020,2008,1990,2022,2006,1986,2002,1981,2001,1986,2006,2025,1980,1980,2006,1988,2006,1988,2019,2020,2023,2021,1977,1981,2021,2015,1998,2024,2021,1992,1992,2006,2008,2009,1987,2008,2013,1976,1989,1981,2016,2001,1979,1987,1985,2009,1995,1978,1996,1999,2001,2010,2003,2019,2018,1988,2025,1976,2018,1975,2016,1999,1993,2003,1996,2007,2009,2018,2000,1981,1984,2000,2022,1986,2021,1979,2004,2004,1981,2015,1991,1978,1988,2017,1983,2017,2019,2019,2024,1981,1995,1999,1999,2022,1994,1988,2024,2021,2006,1994,1975,2009,2006,1976,2023,1986,2009,1979,2012,1976,2023,1984,1995,2020,1982,1984,2005,2014,1985,2022,2025,2002,1997,2007,1992,2009,2014,2002,1982,1995,1984,1984,2000,2019,1987,1976,1999,1991,2024,2022,1993,1998,1996,2011,1997,1982,1994,1983,1978,2009,1976,1994,1977,2004,2006,2015,1987,1983,1996,1985,2025,1991,2016,1984,2000,1986,1983,1978,1981,1987,2010,2015,2015,2006,1977,1997,1986,2019,2019,1990,2012,1985,1995,1995,2008,2023,2014,1976,1993,2015,1983,1982,1976,1987,1995,2007,2004,1992,2018,2020,1977,2017,1999,1985,2000,1985], variant=\"matrix\", width=1320, height=880")]
pub fn render(cfg: &CirclePackConfig) -> String {
    let n = cfg
        .labels
        .len()
        .min(cfg.parents.len())
        .min(cfg.categories.len())
        .min(cfg.values.len());
    if n == 0 {
        return String::new();
    }

    let (region_order, region_idx) = ordered_by_count(&cfg.parents[..n]);
    let (orbit_order, orbit_idx) = ordered_by_count(&cfg.categories[..n]);

    let cat2_vals: Vec<String> = (0..n)
        .map(|i| {
            let s = cfg.categories2.get(i).map(|s| s.as_str()).unwrap_or("");
            if s.is_empty() { "Other".to_string() } else { s.to_string() }
        })
        .collect();
    let (cat_order, cat_idx) = ordered_by_count(&cat2_vals);

    let sats: Vec<Sat> = (0..n)
        .map(|i| {
            let ci = *cat_idx.get(&cat2_vals[i]).unwrap_or(&0);
            let raw_sym = cfg.symbols.get(i).map(|s| s.as_str()).unwrap_or("");
            let sym = if raw_sym.is_empty() { "circle" } else { raw_sym };
            Sat {
                name: cfg.labels[i].as_str(),
                region: *region_idx.get(&cfg.parents[i]).unwrap_or(&0),
                orbit: *orbit_idx.get(&cfg.categories[i]).unwrap_or(&0),
                cat: ci,
                sym,
                mass: cfg.values[i].max(0.0),
                shade: cfg.color_values.get(i).copied(),
            }
        })
        .collect();

    let n_cols = region_order.len();
    let n_rows = orbit_order.len();
    if n_cols == 0 || n_rows == 0 {
        return String::new();
    }

    let mass_max = sats.iter().map(|s| s.mass).fold(0.0_f64, f64::max).max(1.0);
    let radii: Vec<f64> = sats
        .iter()
        .map(|s| R_MIN + (s.mass / mass_max).sqrt() * (R_MAX - R_MIN))
        .collect();

    let shades: Vec<f64> = sats.iter().filter_map(|s| s.shade).collect();
    let has_shade = !shades.is_empty();
    let shade_min = shades.iter().cloned().fold(f64::INFINITY, f64::min);
    let shade_max = shades.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let shade_span = (shade_max - shade_min).max(1e-9);

    let mut cells: Vec<Vec<usize>> = vec![Vec::new(); n_cols * n_rows];
    for (i, s) in sats.iter().enumerate() {
        cells[s.orbit * n_cols + s.region].push(i);
    }
    let col_counts: Vec<usize> = (0..n_cols)
        .map(|c| sats.iter().filter(|s| s.region == c).count())
        .collect();

    let title_h = if cfg.title.is_empty() { 0.0 } else { 30.0 };
    let plot_l = PAD + ROWLABEL_W;
    let plot_t = PAD + title_h + COLHEAD_H;
    let plot_w = (cfg.width as f64 - plot_l - PAD).max(160.0);
    let plot_h = (cfg.height as f64 - plot_t - PAD - LEGEND_H - LEGEND_GAP).max(160.0);

    let col_w = proportional_extents(&col_counts, plot_w, 4.0);
    let mut col_x = vec![0.0_f64; n_cols];
    let mut acc = plot_l;
    for i in 0..n_cols {
        col_x[i] = acc;
        acc += col_w[i] + 4.0;
    }

    let mut col_row_h: Vec<Vec<f64>> = Vec::with_capacity(n_cols);
    let mut col_row_y: Vec<Vec<f64>> = Vec::with_capacity(n_cols);
    for c in 0..n_cols {
        let counts: Vec<usize> = (0..n_rows).map(|r| cells[r * n_cols + c].len()).collect();
        let heights = proportional_extents(&counts, plot_h, 3.0);
        let mut ys = vec![0.0_f64; n_rows];
        let mut acc = plot_t;
        for r in 0..n_rows {
            ys[r] = acc;
            acc += heights[r] + 3.0;
        }
        col_row_y.push(ys);
        col_row_h.push(heights);
    }

    let mut buf = Vec::<u8>::with_capacity(n * 200 + 8192);
    svg_open(&mut buf, cfg.width, cfg.height);

    if !cfg.title.is_empty() {
        push_b(&mut buf, b"<text x=\"");
        push_f2(&mut buf, cfg.width as f64 / 2.0);
        push_b(&mut buf, b"\" y=\"22\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"14\" font-weight=\"700\" fill=\"#");
        push_b(&mut buf, INK.as_bytes());
        push_b(&mut buf, b"\" letter-spacing=\"1\">");
        escape_xml(&mut buf, cfg.title);
        push_b(&mut buf, b"</text>");
    }

    for (i, name) in region_order.iter().enumerate() {
        let cx = col_x[i] + col_w[i] / 2.0;
        let cy = plot_t - COLHEAD_H;
        push_b(&mut buf, b"<text x=\"");
        push_f2(&mut buf, cx);
        push_b(&mut buf, b"\" y=\"");
        push_f2(&mut buf, cy + 16.0);
        push_b(&mut buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"12\" font-weight=\"700\" fill=\"#");
        push_b(&mut buf, INK.as_bytes());
        push_b(&mut buf, b"\">");
        escape_xml(&mut buf, name);
        push_b(&mut buf, b"</text>");
        push_b(&mut buf, b"<text x=\"");
        push_f2(&mut buf, cx);
        push_b(&mut buf, b"\" y=\"");
        push_f2(&mut buf, cy + 30.0);
        push_b(&mut buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"9.5\" fill=\"#");
        push_b(&mut buf, SUB.as_bytes());
        push_b(&mut buf, b"\">");
        push_b(&mut buf, col_counts[i].to_string().as_bytes());
        push_b(&mut buf, if col_counts[i] == 1 { b" sat" } else { b" sats" });
        push_b(&mut buf, b"</text>");
    }

    for (j, name) in orbit_order.iter().enumerate() {
        let ry = col_row_y[0][j] + col_row_h[0][j] / 2.0 + 4.0;
        push_b(&mut buf, b"<text x=\"");
        push_f2(&mut buf, plot_l - 12.0);
        push_b(&mut buf, b"\" y=\"");
        push_f2(&mut buf, ry);
        push_b(&mut buf, b"\" text-anchor=\"end\" font-family=\"Arial,sans-serif\" font-size=\"11.5\" font-weight=\"600\" fill=\"#");
        push_b(&mut buf, STRONG.as_bytes());
        push_b(&mut buf, b"\">");
        escape_xml(&mut buf, name);
        push_b(&mut buf, b"</text>");
    }

    for k in 0..n_rows.saturating_sub(1) {
        let ys: Vec<f64> = (0..n_cols).map(|c| col_row_y[c][k] + col_row_h[c][k] + 1.5).collect();
        staircase_path(&mut buf, &col_x, &col_w, &ys);
    }

    let white_hex = hex6(0xffffff);
    for r in 0..n_rows {
        for c in 0..n_cols {
            let members = &cells[r * n_cols + c];
            if members.is_empty() {
                continue;
            }
            let mut order = members.clone();
            order.sort_by(|&a, &b| radii[b].partial_cmp(&radii[a]).unwrap_or(std::cmp::Ordering::Equal));
            let base_radii: Vec<f64> = order.iter().map(|&i| radii[i]).collect();

            let cell_w = col_w[c];
            let cell_h = col_row_h[c][r];
            let avail_w = cell_w * 0.94;
            let avail_h = cell_h * 0.90;

            let total_area: f64 = base_radii.iter().map(|r| r * r * std::f64::consts::PI).sum();
            let k = ((avail_w * avail_h * 0.62) / total_area.max(1e-6)).sqrt().clamp(0.3, 4.5);
            let seed_radii: Vec<f64> = base_radii.iter().map(|&r| r * k).collect();
            let seed_pos = pack_local(&seed_radii, (0.42 * k).max(0.12));

            let mut min_x = f64::MAX;
            let mut max_x = f64::MIN;
            let mut min_y = f64::MAX;
            let mut max_y = f64::MIN;
            for (i, &(px, py)) in seed_pos.iter().enumerate() {
                let rr = seed_radii[i];
                min_x = min_x.min(px - rr);
                max_x = max_x.max(px + rr);
                min_y = min_y.min(py - rr);
                max_y = max_y.max(py + rr);
            }
            let bbox_w = (max_x - min_x).max(1e-6);
            let bbox_h = (max_y - min_y).max(1e-6);
            let fit = ((avail_w / bbox_w).min(avail_h / bbox_h)).clamp(0.35, 1.9);
            let mid_x = (min_x + max_x) / 2.0;
            let mid_y = (min_y + max_y) / 2.0;

            let raw_pos = seed_pos;
            let local_radii: Vec<f64> = seed_radii.iter().map(|&r| r * fit).collect();

            let cell_cx = col_x[c] + cell_w / 2.0;
            let cell_cy = col_row_y[c][r] + cell_h / 2.0;

            for (oi, &ci) in order.iter().enumerate() {
                let (lx, ly) = raw_pos[oi];
                let px = cell_cx + (lx - mid_x) * fit;
                let py = cell_cy + (ly - mid_y) * fit;
                let rr = local_radii[oi].max(0.6);
                let s = &sats[ci];
                let col = palette_color(cfg.palette, s.cat);
                let hx = hex6(col);
                let opacity = match s.shade {
                    Some(v) if has_shade => 0.42 + ((v - shade_min) / shade_span).clamp(0.0, 1.0) * 0.50,
                    _ => 0.88,
                };
                push_b(&mut buf, b"<g data-idx=\"");
                push_i(&mut buf, ci as i32);
                push_b(&mut buf, b"\">");
                push_b(&mut buf, b"<circle cx=\"");
                push_f2(&mut buf, px);
                push_b(&mut buf, b"\" cy=\"");
                push_f2(&mut buf, py);
                push_b(&mut buf, b"\" r=\"");
                push_f2(&mut buf, rr);
                push_b(&mut buf, b"\" fill=\"#");
                buf.extend_from_slice(&hx);
                push_b(&mut buf, b"\" fill-opacity=\"");
                push_f2(&mut buf, opacity);
                push_b(&mut buf, b"\" stroke=\"#");
                buf.extend_from_slice(&white_hex);
                push_b(&mut buf, b"\" stroke-width=\"0.6\"/>");
                if s.sym != "circle" && rr > 3.2 {
                    draw_glyph(&mut buf, s.sym, px, py, rr * 0.62);
                }
                push_b(&mut buf, b"</g>");
            }
        }
    }

    let leg_y0 = plot_t + plot_h + LEGEND_GAP;
    let leg_x0 = plot_l;
    let n_seg = if has_shade { 4 } else { 3 };
    let seg_w = plot_w / n_seg as f64;

    push_b(&mut buf, b"<g data-legend=\"matrix\">");

    push_b(&mut buf, b"<line x1=\"");
    push_f2(&mut buf, leg_x0);
    push_b(&mut buf, b"\" y1=\"");
    push_f2(&mut buf, leg_y0 - 2.0);
    push_b(&mut buf, b"\" x2=\"");
    push_f2(&mut buf, leg_x0 + plot_w);
    push_b(&mut buf, b"\" y2=\"");
    push_f2(&mut buf, leg_y0 - 2.0);
    push_b(&mut buf, b"\" stroke=\"#");
    push_b(&mut buf, RULE.as_bytes());
    push_b(&mut buf, b"\" stroke-width=\"1\"/>");

    {
        legend_title(&mut buf, leg_x0, leg_y0 + 20.0, "SIZE (KG)");
        let refs: [f64; 3] = [100.0, 1000.0, 5000.0];
        let mut dx = leg_x0 + 10.0;
        let dy = leg_y0 + 56.0;
        for &m in refs.iter() {
            let t = (m / mass_max).clamp(0.0, 1.0).sqrt();
            let r = R_MIN + t * (R_MAX - R_MIN);
            legend_dot(&mut buf, dx, dy, r, NEUTRAL_DOT.as_bytes(), 0.85);
            let label = if m >= 1000.0 {
                format!("{:.0}k", m / 1000.0)
            } else {
                format!("{:.0}", m)
            };
            push_b(&mut buf, b"<text x=\"");
            push_f2(&mut buf, dx);
            push_b(&mut buf, b"\" y=\"");
            push_f2(&mut buf, dy + R_MAX + 13.0);
            push_b(&mut buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"9\" fill=\"#");
            push_b(&mut buf, SUB.as_bytes());
            push_b(&mut buf, b"\">");
            push_b(&mut buf, label.as_bytes());
            push_b(&mut buf, b"</text>");
            dx += 36.0;
        }
    }

    {
        let sx = leg_x0 + seg_w;
        legend_title(&mut buf, sx, leg_y0 + 20.0, "CATEGORY");
        let mut fx = sx;
        let mut fy = leg_y0 + 38.0;
        let max_x = sx + seg_w - 10.0;
        for (ci, name) in cat_order.iter().enumerate() {
            let w_est = 16.0 + name.len() as f64 * 5.6;
            if fx + w_est > max_x && fx > sx {
                fx = sx;
                fy += 17.0;
            }
            let col = palette_color(cfg.palette, ci);
            legend_dot(&mut buf, fx + 4.0, fy, 4.0, &hex6(col), 0.92);
            push_b(&mut buf, b"<text x=\"");
            push_f2(&mut buf, fx + 12.0);
            push_b(&mut buf, b"\" y=\"");
            push_f2(&mut buf, fy + 3.0);
            push_b(&mut buf, b"\" font-family=\"Arial,sans-serif\" font-size=\"10\" fill=\"#");
            push_b(&mut buf, STRONG.as_bytes());
            push_b(&mut buf, b"\">");
            escape_xml(&mut buf, name);
            push_b(&mut buf, b"</text>");
            fx += w_est;
        }
    }

    {
        let sx = leg_x0 + seg_w * 2.0;
        legend_title(&mut buf, sx, leg_y0 + 20.0, "CLASS");
        let sym_names: Vec<String> = sats.iter().map(|s| s.sym.to_string()).collect();
        let (sym_order, _) = ordered_by_count(&sym_names);
        let mut fx = sx;
        let mut fy = leg_y0 + 42.0;
        let max_x = sx + seg_w - 10.0;
        for sym in sym_order.iter() {
            let label = if sym == "circle" { "Commercial".to_string() } else { cap_label(sym) };
            let w_est = 20.0 + label.len() as f64 * 5.6;
            if fx + w_est > max_x && fx > sx {
                fx = sx;
                fy += 20.0;
            }
            legend_dot(&mut buf, fx + 6.0, fy - 3.0, 6.5, NEUTRAL_DOT.as_bytes(), 0.85);
            draw_glyph(&mut buf, sym, fx + 6.0, fy - 3.0, 4.6);
            push_b(&mut buf, b"<text x=\"");
            push_f2(&mut buf, fx + 16.0);
            push_b(&mut buf, b"\" y=\"");
            push_f2(&mut buf, fy);
            push_b(&mut buf, b"\" font-family=\"Arial,sans-serif\" font-size=\"10\" fill=\"#");
            push_b(&mut buf, STRONG.as_bytes());
            push_b(&mut buf, b"\">");
            escape_xml(&mut buf, &label);
            push_b(&mut buf, b"</text>");
            fx += w_est;
        }
    }

    if has_shade {
        let sx = leg_x0 + seg_w * 3.0;
        legend_title(&mut buf, sx, leg_y0 + 20.0, "LAUNCH DATE");
        let dy = leg_y0 + 52.0;
        legend_dot(&mut buf, sx + 8.0, dy, 6.0, NEUTRAL_DOT.as_bytes(), 0.40);
        legend_dot(&mut buf, sx + 40.0, dy, 6.0, NEUTRAL_DOT.as_bytes(), 0.92);
        push_b(&mut buf, b"<text x=\"");
        push_f2(&mut buf, sx + 8.0);
        push_b(&mut buf, b"\" y=\"");
        push_f2(&mut buf, dy + 20.0);
        push_b(&mut buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"9\" fill=\"#");
        push_b(&mut buf, SUB.as_bytes());
        push_b(&mut buf, b"\">");
        push_b(&mut buf, format!("{:.0}", shade_min).as_bytes());
        push_b(&mut buf, b"</text>");
        push_b(&mut buf, b"<text x=\"");
        push_f2(&mut buf, sx + 40.0);
        push_b(&mut buf, b"\" y=\"");
        push_f2(&mut buf, dy + 20.0);
        push_b(&mut buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"9\" fill=\"#");
        push_b(&mut buf, SUB.as_bytes());
        push_b(&mut buf, b"\">");
        push_b(&mut buf, format!("{:.0}", shade_max).as_bytes());
        push_b(&mut buf, b"</text>");
    }

    push_b(&mut buf, b"</g>");

    push_b(&mut buf, b"</svg>");
    let svg = unsafe { String::from_utf8_unchecked(buf) };

    let mut slots: Vec<HoverSlot> = Vec::with_capacity(n);
    for s in sats.iter() {
        let mut slot = HoverSlot::new(s.name.to_string())
            .kv("Region", region_order[s.region].clone())
            .kv("Orbit", orbit_order[s.orbit].clone())
            .kv("Category", cat_order[s.cat].clone())
            .kv("Class", if s.sym == "circle" { "Commercial".to_string() } else { cap_label(s.sym) })
            .kv("Mass", format!("{:.0} kg", s.mass));
        if let Some(v) = s.shade {
            slot = slot.kv("Launch", format!("{:.0}", v));
        }
        slots.push(slot);
    }

    build_chart_html(cfg.title, &svg, &slots_to_json(&slots))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cfg<'a>(
        labels: &'a [String],
        parents: &'a [String],
        categories: &'a [String],
        categories2: &'a [String],
        symbols: &'a [String],
        values: &'a [f64],
        color_values: &'a [f64],
    ) -> CirclePackConfig<'a> {
        CirclePackConfig {
            title: "Test",
            labels,
            parents,
            categories,
            categories2,
            symbols,
            values,
            color_values,
            width: 1300,
            height: 860,
            ..CirclePackConfig::default()
        }
    }

    fn synth(n: usize) -> (Vec<String>, Vec<String>, Vec<String>, Vec<String>, Vec<String>, Vec<f64>, Vec<f64>) {
        let regions = ["U.S.", "China", "Russia"];
        let orbits = ["Low Earth Orbit", "Geosynchronous Orbit"];
        let cats = ["Communications", "Research", "Navigation"];
        let syms = ["circle", "star", "diamond", "triangle"];
        let mut labels = Vec::with_capacity(n);
        let mut parents = Vec::with_capacity(n);
        let mut categories = Vec::with_capacity(n);
        let mut categories2 = Vec::with_capacity(n);
        let mut symbols = Vec::with_capacity(n);
        let mut values = Vec::with_capacity(n);
        let mut color_values = Vec::with_capacity(n);
        for i in 0..n {
            labels.push(format!("Sat-{i}"));
            parents.push(regions[i % regions.len()].to_string());
            categories.push(orbits[i % orbits.len()].to_string());
            categories2.push(cats[i % cats.len()].to_string());
            symbols.push(syms[i % syms.len()].to_string());
            values.push(((i % 50) + 1) as f64 * 40.0);
            color_values.push(1980.0 + (i % 45) as f64);
        }
        (labels, parents, categories, categories2, symbols, values, color_values)
    }

    fn skewed(n: usize) -> (Vec<String>, Vec<String>, Vec<String>, Vec<String>, Vec<String>, Vec<f64>, Vec<f64>) {
        let mut d = synth(n);
        for i in 0..d.1.len() {
            if i % 5 != 0 {
                d.1[i] = "U.S.".to_string();
                d.2[i] = "Low Earth Orbit".to_string();
            }
        }
        d
    }

    #[test]
    #[ignore]
    fn write_preview_asset() {
        use crate::plot::chart_demo_registry::{iter_entries, render_demo_html};
        for entry in iter_entries() {
            if !entry.file.replace('\\', "/").ends_with("circle_pack/matrix.rs") {
                continue;
            }
            let html = render_demo_html(entry).expect("demo html");
            std::fs::write("docs/previews/circle_pack-matrix.html", html).unwrap();
        }
    }

    #[test]
    fn renders_one_hoverable_mark_per_point() {
        let (labels, parents, categories, categories2, symbols, values, color_values) = synth(90);
        let html = render(&cfg(&labels, &parents, &categories, &categories2, &symbols, &values, &color_values));
        assert!(!html.is_empty());
        assert_eq!(html.matches("<g data-idx=\"").count(), 90);
    }

    #[test]
    fn draws_both_axes_as_real_category_labels() {
        let (labels, parents, categories, categories2, symbols, values, color_values) = synth(30);
        let html = render(&cfg(&labels, &parents, &categories, &categories2, &symbols, &values, &color_values));
        assert!(html.contains("U.S."));
        assert!(html.contains("China"));
        assert!(html.contains("Russia"));
        assert!(html.contains("Low Earth Orbit"));
        assert!(html.contains("Geosynchronous Orbit"));
    }

    #[test]
    fn every_point_is_a_circle_and_non_commercial_classes_get_a_glyph_overlay() {
        let (labels, parents, categories, categories2, symbols, values, color_values) = synth(80);
        let html = render(&cfg(&labels, &parents, &categories, &categories2, &symbols, &values, &color_values));
        let marks = html.matches("<g data-idx=\"").count();
        assert_eq!(marks, 80);
        assert!(html.matches("<circle cx=\"").count() >= marks);
        assert!(html.contains("<polygon points=\""));
    }

    #[test]
    fn a_lopsided_column_gets_its_own_row_split_producing_a_stepped_boundary() {
        let (labels, parents, categories, categories2, symbols, values, color_values) = skewed(120);
        let html = render(&cfg(&labels, &parents, &categories, &categories2, &symbols, &values, &color_values));
        assert!(html.contains("<path d=\"M"));
    }

    #[test]
    fn defaults_to_the_same_white_background_as_every_other_seraplot_chart() {
        let (labels, parents, categories, categories2, symbols, values, color_values) = synth(20);
        let html = render(&cfg(&labels, &parents, &categories, &categories2, &symbols, &values, &color_values));
        assert!(html.contains("class=\"sp-bg\""));
        assert!(!html.contains("fill=\"#0b1220\""));
    }

    #[test]
    fn the_legend_is_wrapped_for_no_legend_and_the_chart_carries_no_outer_frame() {
        let (labels, parents, categories, categories2, symbols, values, color_values) = synth(40);
        let html = render(&cfg(&labels, &parents, &categories, &categories2, &symbols, &values, &color_values));
        assert!(html.contains("data-legend=\"matrix\""));
        assert!(!html.contains("rx=\"10\""));
    }

    #[test]
    fn empty_input_returns_empty_string() {
        let empty_s: Vec<String> = vec![];
        let empty_v: Vec<f64> = vec![];
        let html = render(&cfg(&empty_s, &empty_s, &empty_s, &empty_s, &empty_s, &empty_v, &empty_v));
        assert!(html.is_empty());
    }

    #[test]
    fn perf_rendering_a_thousand_satellites_stays_fast() {
        let (labels, parents, categories, categories2, symbols, values, color_values) = synth(1000);
        let c = cfg(&labels, &parents, &categories, &categories2, &symbols, &values, &color_values);
        let start = std::time::Instant::now();
        let html = render(&c);
        let elapsed = start.elapsed();
        assert!(elapsed.as_millis() < 2500, "rendering took too long: {elapsed:?}");
        assert!(!html.is_empty());
    }

    #[test]
    fn perf_rendering_a_thousand_satellites_crammed_into_one_lopsided_column_stays_fast() {
        let (labels, parents, categories, categories2, symbols, values, color_values) = skewed(1000);
        let c = cfg(&labels, &parents, &categories, &categories2, &symbols, &values, &color_values);
        let start = std::time::Instant::now();
        let html = render(&c);
        let elapsed = start.elapsed();
        assert!(elapsed.as_millis() < 3500, "rendering took too long: {elapsed:?}");
        assert!(!html.is_empty());
    }
}
