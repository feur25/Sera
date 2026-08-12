use super::config::BubbleConfig;
use crate::html::hover::{build_chart_html, slots_to_json, HoverSlot};
use crate::plot::statistical::common::{escape_xml, hex6, polar_point, push_b, push_f2, push_i, svg_open};
use std::f64::consts::PI;

#[crate::chart_demo(
    "labels=[\"Monaco GP #1\",\"Baku GP #2\",\"Singapore GP #3\",\"Monza GP #4\",\"Austin GP #5\",\"Shanghai GP #6\",\"Sochi GP #7\",\"Shanghai GP #8\",\"Mexico City GP #9\",\"Spa GP #10\",\"Suzuka GP #11\",\"Mexico City GP #12\",\"Nurburgring GP #13\",\"Shanghai GP #14\",\"Yas Marina GP #15\",\"Monza GP #16\",\"Zolder GP #17\",\"Montreal GP #18\",\"Suzuka GP #19\",\"Mexico City GP #20\",\"Montreal GP #21\",\"Budapest GP #22\",\"Baku GP #23\",\"Barcelona GP #24\",\"Yas Marina GP #25\",\"Interlagos GP #26\",\"Shanghai GP #27\",\"Imola GP #28\",\"Austin GP #29\",\"Austin GP #30\",\"Sochi GP #31\",\"Montreal GP #32\",\"Singapore GP #33\",\"Interlagos GP #34\",\"Monza GP #35\",\"Spa GP #36\",\"Singapore GP #37\",\"Imola GP #38\",\"Baku GP #39\",\"Melbourne GP #40\",\"Imola GP #41\",\"Suzuka GP #42\",\"Melbourne GP #43\",\"Silverstone GP #44\",\"Baku GP #45\",\"Imola GP #46\",\"Melbourne GP #47\",\"Interlagos GP #48\",\"Baku GP #49\",\"Mexico City GP #50\",\"Imola GP #51\",\"Hockenheim GP #52\",\"Budapest GP #53\",\"Yas Marina GP #54\",\"Interlagos GP #55\",\"Singapore GP #56\",\"Budapest GP #57\",\"Spa GP #58\",\"Sochi GP #59\",\"Suzuka GP #60\",\"Sochi GP #61\",\"Montreal GP #62\",\"Suzuka GP #63\",\"Yas Marina GP #64\",\"Baku GP #65\",\"Imola GP #66\",\"Silverstone GP #67\",\"Melbourne GP #68\",\"Baku GP #69\",\"Hockenheim GP #70\",\"Shanghai GP #71\",\"Monaco GP #72\",\"Budapest GP #73\",\"Barcelona GP #74\",\"Melbourne GP #75\",\"Suzuka GP #76\",\"Suzuka GP #77\",\"Sepang GP #78\",\"Sepang GP #79\",\"Sepang GP #80\",\"Sochi GP #81\",\"Mexico City GP #82\",\"Singapore GP #83\",\"Austin GP #84\",\"Budapest GP #85\",\"Jeddah GP #86\",\"Sepang GP #87\",\"Monaco GP #88\",\"Spa GP #89\",\"Austin GP #90\",\"Suzuka GP #91\",\"Monaco GP #92\",\"Interlagos GP #93\",\"Melbourne GP #94\",\"Suzuka GP #95\",\"Nurburgring GP #96\",\"Mexico City GP #97\",\"Barcelona GP #98\",\"Singapore GP #99\",\"Zolder GP #100\",\"Spa GP #101\",\"Zandvoort GP #102\",\"Yas Marina GP #103\",\"Monaco GP #104\",\"Austin GP #105\",\"Austin GP #106\",\"Yas Marina GP #107\",\"Spa GP #108\",\"Baku GP #109\",\"Sepang GP #110\",\"Monaco GP #111\",\"Spa GP #112\",\"Interlagos GP #113\",\"Mexico City GP #114\",\"Suzuka GP #115\",\"Silverstone GP #116\",\"Suzuka GP #117\",\"Spa GP #118\",\"Melbourne GP #119\",\"Melbourne GP #120\",\"Imola GP #121\",\"Singapore GP #122\",\"Monza GP #123\",\"Mexico City GP #124\",\"Spa GP #125\",\"Austin GP #126\",\"Zandvoort GP #127\",\"Budapest GP #128\",\"Zolder GP #129\",\"Zolder GP #130\",\"Silverstone GP #131\",\"Melbourne GP #132\",\"Silverstone GP #133\",\"Montreal GP #134\",\"Interlagos GP #135\",\"Mexico City GP #136\",\"Jeddah GP #137\",\"Zolder GP #138\",\"Singapore GP #139\",\"Jeddah GP #140\",\"Baku GP #141\",\"Melbourne GP #142\",\"Baku GP #143\",\"Austin GP #144\",\"Silverstone GP #145\",\"Spa GP #146\",\"Singapore GP #147\",\"Melbourne GP #148\",\"Mexico City GP #149\",\"Austin GP #150\",\"Hockenheim GP #151\",\"Mexico City GP #152\",\"Zolder GP #153\",\"Budapest GP #154\",\"Singapore GP #155\",\"Monaco GP #156\",\"Zandvoort GP #157\",\"Zandvoort GP #158\",\"Austin GP #159\",\"Budapest GP #160\",\"Jeddah GP #161\",\"Yas Marina GP #162\",\"Baku GP #163\",\"Zandvoort GP #164\",\"Interlagos GP #165\",\"Melbourne GP #166\",\"Baku GP #167\",\"Austin GP #168\",\"Melbourne GP #169\",\"Budapest GP #170\",\"Imola GP #171\",\"Silverstone GP #172\",\"Spa GP #173\",\"Jeddah GP #174\",\"Montreal GP #175\",\"Imola GP #176\",\"Monza GP #177\",\"Zandvoort GP #178\",\"Hockenheim GP #179\",\"Imola GP #180\",\"Baku GP #181\",\"Yas Marina GP #182\",\"Monza GP #183\",\"Mexico City GP #184\",\"Monza GP #185\",\"Budapest GP #186\",\"Mexico City GP #187\",\"Singapore GP #188\",\"Montreal GP #189\",\"Interlagos GP #190\",\"Interlagos GP #191\",\"Budapest GP #192\",\"Spa GP #193\",\"Hockenheim GP #194\",\"Jeddah GP #195\",\"Barcelona GP #196\",\"Nurburgring GP #197\",\"Zolder GP #198\",\"Singapore GP #199\",\"Zandvoort GP #200\",\"Budapest GP #201\",\"Baku GP #202\",\"Jeddah GP #203\",\"Silverstone GP #204\",\"Baku GP #205\",\"Hockenheim GP #206\",\"Melbourne GP #207\",\"Mexico City GP #208\",\"Budapest GP #209\",\"Interlagos GP #210\",\"Zandvoort GP #211\",\"Singapore GP #212\",\"Monza GP #213\",\"Montreal GP #214\",\"Silverstone GP #215\",\"Yas Marina GP #216\",\"Budapest GP #217\",\"Yas Marina GP #218\",\"Silverstone GP #219\",\"Barcelona GP #220\",\"Yas Marina GP #221\",\"Barcelona GP #222\",\"Sochi GP #223\",\"Interlagos GP #224\",\"Baku GP #225\",\"Mexico City GP #226\",\"Melbourne GP #227\",\"Barcelona GP #228\",\"Monaco GP #229\",\"Zolder GP #230\",\"Monza GP #231\",\"Jeddah GP #232\",\"Yas Marina GP #233\",\"Yas Marina GP #234\",\"Imola GP #235\",\"Suzuka GP #236\",\"Imola GP #237\",\"Zolder GP #238\",\"Baku GP #239\",\"Monza GP #240\",\"Yas Marina GP #241\",\"Jeddah GP #242\",\"Baku GP #243\",\"Monza GP #244\",\"Mexico City GP #245\",\"Zandvoort GP #246\",\"Hockenheim GP #247\",\"Zandvoort GP #248\",\"Yas Marina GP #249\",\"Silverstone GP #250\",\"Yas Marina GP #251\",\"Hockenheim GP #252\",\"Yas Marina GP #253\",\"Mexico City GP #254\",\"Shanghai GP #255\",\"Imola GP #256\",\"Suzuka GP #257\",\"Monza GP #258\",\"Nurburgring GP #259\",\"Zolder GP #260\",\"Spa GP #261\",\"Budapest GP #262\",\"Singapore GP #263\",\"Baku GP #264\",\"Baku GP #265\",\"Imola GP #266\",\"Hockenheim GP #267\",\"Jeddah GP #268\",\"Montreal GP #269\",\"Interlagos GP #270\",\"Zolder GP #271\",\"Baku GP #272\",\"Monza GP #273\",\"Suzuka GP #274\",\"Yas Marina GP #275\",\"Budapest GP #276\",\"Yas Marina GP #277\",\"Singapore GP #278\",\"Monaco GP #279\",\"Shanghai GP #280\",\"Silverstone GP #281\",\"Suzuka GP #282\",\"Sochi GP #283\",\"Spa GP #284\",\"Singapore GP #285\",\"Austin GP #286\",\"Zandvoort GP #287\",\"Sepang GP #288\",\"Baku GP #289\",\"Suzuka GP #290\",\"Silverstone GP #291\",\"Baku GP #292\",\"Budapest GP #293\",\"Singapore GP #294\",\"Singapore GP #295\",\"Monaco GP #296\",\"Yas Marina GP #297\",\"Sochi GP #298\",\"Monaco GP #299\",\"Budapest GP #300\"], x_values=[1.0,2.0,3.0,4.0,5.0,6.0,7.0,8.0,9.0,10.0,11.0,12.0,13.0,14.0,15.0,16.0,17.0,18.0,19.0,20.0,21.0,22.0,23.0,24.0,25.0,26.0,27.0,28.0,29.0,30.0,31.0,32.0,33.0,34.0,35.0,36.0,37.0,38.0,39.0,40.0,41.0,42.0,43.0,44.0,45.0,46.0,47.0,48.0,49.0,50.0,51.0,52.0,53.0,54.0,55.0,56.0,57.0,58.0,59.0,60.0,61.0,62.0,63.0,64.0,65.0,66.0,67.0,68.0,69.0,70.0,71.0,72.0,73.0,74.0,75.0,76.0,77.0,78.0,79.0,80.0,81.0,82.0,83.0,84.0,85.0,86.0,87.0,88.0,89.0,90.0,91.0,92.0,93.0,94.0,95.0,96.0,97.0,98.0,99.0,100.0,101.0,102.0,103.0,104.0,105.0,106.0,107.0,108.0,109.0,110.0,111.0,112.0,113.0,114.0,115.0,116.0,117.0,118.0,119.0,120.0,121.0,122.0,123.0,124.0,125.0,126.0,127.0,128.0,129.0,130.0,131.0,132.0,133.0,134.0,135.0,136.0,137.0,138.0,139.0,140.0,141.0,142.0,143.0,144.0,145.0,146.0,147.0,148.0,149.0,150.0,151.0,152.0,153.0,154.0,155.0,156.0,157.0,158.0,159.0,160.0,161.0,162.0,163.0,164.0,165.0,166.0,167.0,168.0,169.0,170.0,171.0,172.0,173.0,174.0,175.0,176.0,177.0,178.0,179.0,180.0,181.0,182.0,183.0,184.0,185.0,186.0,187.0,188.0,189.0,190.0,191.0,192.0,193.0,194.0,195.0,196.0,197.0,198.0,199.0,200.0,201.0,202.0,203.0,204.0,205.0,206.0,207.0,208.0,209.0,210.0,211.0,212.0,213.0,214.0,215.0,216.0,217.0,218.0,219.0,220.0,221.0,222.0,223.0,224.0,225.0,226.0,227.0,228.0,229.0,230.0,231.0,232.0,233.0,234.0,235.0,236.0,237.0,238.0,239.0,240.0,241.0,242.0,243.0,244.0,245.0,246.0,247.0,248.0,249.0,250.0,251.0,252.0,253.0,254.0,255.0,256.0,257.0,258.0,259.0,260.0,261.0,262.0,263.0,264.0,265.0,266.0,267.0,268.0,269.0,270.0,271.0,272.0,273.0,274.0,275.0,276.0,277.0,278.0,279.0,280.0,281.0,282.0,283.0,284.0,285.0,286.0,287.0,288.0,289.0,290.0,291.0,292.0,293.0,294.0,295.0,296.0,297.0,298.0,299.0,300.0], sizes=[9.1,10.7,15.7,7.5,17.1,17.8,24.2,19.7,13.5,16.9,28.7,20.9,24.1,17.9,18.3,24.2,27.4,26.6,9.0,9.4,28.7,20.3,28.5,15.3,9.9,12.1,26.2,20.7,19.4,7.4,28.7,16.4,7.6,10.2,6.0,30.7,9.9,15.5,31.8,8.2,12.9,6.6,9.8,19.7,24.1,10.3,26.3,27.1,27.0,11.9,24.8,24.0,30.4,15.5,11.1,29.4,23.0,23.2,25.5,26.5,31.3,30.6,9.3,27.0,31.5,20.3,31.2,30.3,27.5,13.6,12.7,29.7,21.2,29.9,26.4,6.1,18.3,14.5,26.4,12.5,19.2,29.7,19.1,17.8,30.5,30.5,30.5,9.2,12.3,26.4,24.6,29.0,30.8,31.7,17.2,11.1,6.5,6.5,19.3,26.5,29.8,9.4,27.3,29.9,8.3,17.1,22.5,28.3,17.8,30.1,19.7,10.2,14.1,13.5,24.5,25.1,18.3,27.3,27.7,23.9,27.6,16.5,9.4,12.6,27.9,13.3,17.9,12.8,20.2,14.0,15.9,11.2,12.9,25.0,21.2,23.1,16.1,9.9,7.1,22.3,9.6,27.7,21.2,24.0,9.5,27.7,22.3,6.1,19.1,7.7,7.9,11.3,18.8,23.8,22.7,12.6,22.6,23.5,13.6,18.1,11.2,6.5,31.2,11.5,21.1,30.8,19.2,12.0,26.9,13.9,14.2,25.5,30.1,13.5,32.0,17.1,8.6,30.3,19.3,30.9,22.4,20.3,25.0,22.8,30.1,14.9,31.4,13.8,10.4,29.6,29.6,9.6,14.9,12.7,25.5,19.6,7.6,9.3,28.4,12.5,30.8,29.1,18.3,16.2,28.2,8.8,23.7,22.8,29.8,29.9,9.3,24.2,19.6,11.8,13.8,22.8,12.1,24.3,19.0,12.7,25.4,23.7,25.2,31.2,12.0,13.7,10.9,23.3,16.2,9.7,16.2,25.1,14.6,28.6,15.7,6.1,30.8,11.4,27.4,18.3,11.0,6.8,22.3,29.9,29.4,30.9,24.6,6.1,22.5,12.1,30.8,17.2,10.8,27.4,14.5,26.3,25.6,6.9,31.5,12.9,19.0,12.1,23.5,23.3,13.6,25.2,12.4,21.0,31.8,27.0,8.7,27.9,13.6,31.3,15.7,12.8,8.8,11.7,11.3,22.9,14.5,14.1,20.2,16.3,8.4,16.7,30.8,15.3,31.9,24.9,29.4,16.6,27.5,29.7,15.6,13.4,8.8,31.1,30.5,7.4], categories=[\"Monaco\",\"Baku\",\"Singapore\",\"Monza\",\"Austin\",\"Shanghai\",\"Sochi\",\"Shanghai\",\"Mexico City\",\"Spa\",\"Suzuka\",\"Mexico City\",\"Nurburgring\",\"Shanghai\",\"Yas Marina\",\"Monza\",\"Zolder\",\"Montreal\",\"Suzuka\",\"Mexico City\",\"Montreal\",\"Budapest\",\"Baku\",\"Barcelona\",\"Yas Marina\",\"Interlagos\",\"Shanghai\",\"Imola\",\"Austin\",\"Austin\",\"Sochi\",\"Montreal\",\"Singapore\",\"Interlagos\",\"Monza\",\"Spa\",\"Singapore\",\"Imola\",\"Baku\",\"Melbourne\",\"Imola\",\"Suzuka\",\"Melbourne\",\"Silverstone\",\"Baku\",\"Imola\",\"Melbourne\",\"Interlagos\",\"Baku\",\"Mexico City\",\"Imola\",\"Hockenheim\",\"Budapest\",\"Yas Marina\",\"Interlagos\",\"Singapore\",\"Budapest\",\"Spa\",\"Sochi\",\"Suzuka\",\"Sochi\",\"Montreal\",\"Suzuka\",\"Yas Marina\",\"Baku\",\"Imola\",\"Silverstone\",\"Melbourne\",\"Baku\",\"Hockenheim\",\"Shanghai\",\"Monaco\",\"Budapest\",\"Barcelona\",\"Melbourne\",\"Suzuka\",\"Suzuka\",\"Sepang\",\"Sepang\",\"Sepang\",\"Sochi\",\"Mexico City\",\"Singapore\",\"Austin\",\"Budapest\",\"Jeddah\",\"Sepang\",\"Monaco\",\"Spa\",\"Austin\",\"Suzuka\",\"Monaco\",\"Interlagos\",\"Melbourne\",\"Suzuka\",\"Nurburgring\",\"Mexico City\",\"Barcelona\",\"Singapore\",\"Zolder\",\"Spa\",\"Zandvoort\",\"Yas Marina\",\"Monaco\",\"Austin\",\"Austin\",\"Yas Marina\",\"Spa\",\"Baku\",\"Sepang\",\"Monaco\",\"Spa\",\"Interlagos\",\"Mexico City\",\"Suzuka\",\"Silverstone\",\"Suzuka\",\"Spa\",\"Melbourne\",\"Melbourne\",\"Imola\",\"Singapore\",\"Monza\",\"Mexico City\",\"Spa\",\"Austin\",\"Zandvoort\",\"Budapest\",\"Zolder\",\"Zolder\",\"Silverstone\",\"Melbourne\",\"Silverstone\",\"Montreal\",\"Interlagos\",\"Mexico City\",\"Jeddah\",\"Zolder\",\"Singapore\",\"Jeddah\",\"Baku\",\"Melbourne\",\"Baku\",\"Austin\",\"Silverstone\",\"Spa\",\"Singapore\",\"Melbourne\",\"Mexico City\",\"Austin\",\"Hockenheim\",\"Mexico City\",\"Zolder\",\"Budapest\",\"Singapore\",\"Monaco\",\"Zandvoort\",\"Zandvoort\",\"Austin\",\"Budapest\",\"Jeddah\",\"Yas Marina\",\"Baku\",\"Zandvoort\",\"Interlagos\",\"Melbourne\",\"Baku\",\"Austin\",\"Melbourne\",\"Budapest\",\"Imola\",\"Silverstone\",\"Spa\",\"Jeddah\",\"Montreal\",\"Imola\",\"Monza\",\"Zandvoort\",\"Hockenheim\",\"Imola\",\"Baku\",\"Yas Marina\",\"Monza\",\"Mexico City\",\"Monza\",\"Budapest\",\"Mexico City\",\"Singapore\",\"Montreal\",\"Interlagos\",\"Interlagos\",\"Budapest\",\"Spa\",\"Hockenheim\",\"Jeddah\",\"Barcelona\",\"Nurburgring\",\"Zolder\",\"Singapore\",\"Zandvoort\",\"Budapest\",\"Baku\",\"Jeddah\",\"Silverstone\",\"Baku\",\"Hockenheim\",\"Melbourne\",\"Mexico City\",\"Budapest\",\"Interlagos\",\"Zandvoort\",\"Singapore\",\"Monza\",\"Montreal\",\"Silverstone\",\"Yas Marina\",\"Budapest\",\"Yas Marina\",\"Silverstone\",\"Barcelona\",\"Yas Marina\",\"Barcelona\",\"Sochi\",\"Interlagos\",\"Baku\",\"Mexico City\",\"Melbourne\",\"Barcelona\",\"Monaco\",\"Zolder\",\"Monza\",\"Jeddah\",\"Yas Marina\",\"Yas Marina\",\"Imola\",\"Suzuka\",\"Imola\",\"Zolder\",\"Baku\",\"Monza\",\"Yas Marina\",\"Jeddah\",\"Baku\",\"Monza\",\"Mexico City\",\"Zandvoort\",\"Hockenheim\",\"Zandvoort\",\"Yas Marina\",\"Silverstone\",\"Yas Marina\",\"Hockenheim\",\"Yas Marina\",\"Mexico City\",\"Shanghai\",\"Imola\",\"Suzuka\",\"Monza\",\"Nurburgring\",\"Zolder\",\"Spa\",\"Budapest\",\"Singapore\",\"Baku\",\"Baku\",\"Imola\",\"Hockenheim\",\"Jeddah\",\"Montreal\",\"Interlagos\",\"Zolder\",\"Baku\",\"Monza\",\"Suzuka\",\"Yas Marina\",\"Budapest\",\"Yas Marina\",\"Singapore\",\"Monaco\",\"Shanghai\",\"Silverstone\",\"Suzuka\",\"Sochi\",\"Spa\",\"Singapore\",\"Austin\",\"Zandvoort\",\"Sepang\",\"Baku\",\"Suzuka\",\"Silverstone\",\"Baku\",\"Budapest\",\"Singapore\",\"Singapore\",\"Monaco\",\"Yas Marina\",\"Sochi\",\"Monaco\",\"Budapest\"], color_values=[0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,1.0,0.0,0.0,0.0,0.0,0.0,0.0,1.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,1.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,1.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,1.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,1.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,1.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,1.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,1.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,1.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,1.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,1.0,0.0,0.0,0.0,0.0,0.0,0.0,1.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,1.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,1.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,1.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,1.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0], variant=\"radial_rows\", width=620, height=390, min_size=1.5, max_size=8, x_categories=[\"pont fer\",\"pont bois\",\"pont pierre\",\"cheminee bois\",\"cheminee pierre\"]"
)]

pub fn render(cfg: &BubbleConfig) -> String {
    let n = cfg.x_values.len().min(cfg.sizes.len()).min(cfg.categories.len());
    if n == 0 {
        return String::new();
    }

    let mut rows: Vec<String> = Vec::new();
    for c in &cfg.categories[..n] {
        if !rows.iter().any(|r| r == c) {
            rows.push(c.clone());
        }
    }
    let n_rows = rows.len();
    if n_rows == 0 {
        return String::new();
    }
    let row_of = |cat: &str| -> usize { rows.iter().position(|r| r == cat).unwrap_or(0) };

    let mut xmin = f64::INFINITY;
    let mut xmax = f64::NEG_INFINITY;
    let mut smin = f64::INFINITY;
    let mut smax = f64::NEG_INFINITY;
    for i in 0..n {
        xmin = xmin.min(cfg.x_values[i]);
        xmax = xmax.max(cfg.x_values[i]);
        let s = cfg.sizes[i].abs();
        smin = smin.min(s);
        smax = smax.max(s);
    }
    let xr = (xmax - xmin).max(1e-9);
    let sr = (smax - smin).max(1e-9);

    let w = cfg.width;
    let h = cfg.height;
    let label_w = 150.0;
    let top_margin = 34.0;
    let bottom_margin = 22.0;
    let usable_top = top_margin;
    let usable_bottom = h as f64 - bottom_margin;
    let cy = (usable_top + usable_bottom) / 2.0;
    let outer_r = ((usable_bottom - usable_top) / 2.0)
        .min(w as f64 - label_w - 60.0)
        .max(10.0);
    let cx = label_w + 24.0;
    let inner_floor = (outer_r * 0.02).max(3.0);
    let hub_r = outer_r * 0.09;

    let list_top = cy - outer_r;
    let list_bottom = cy + outer_r;
    let row_step = if n_rows > 1 { (list_bottom - list_top) / (n_rows - 1) as f64 } else { 0.0 };

    let mut label_ys = Vec::with_capacity(n_rows);
    let mut row_radii = Vec::with_capacity(n_rows);
    for ri in 0..n_rows {
        let ly = list_top + ri as f64 * row_step;
        label_ys.push(ly);
        row_radii.push((ly - cy).abs().max(inner_floor));
    }

    let start_angle = -PI / 2.0;
    let end_angle = PI / 2.0;
    let mid_angle = 0.0;
    let sweep = PI;

    let angle_of = |x: f64| -> f64 { start_angle + ((x - xmin) / xr) * sweep };

    let color_normal = if cfg.color_low == 0x636EFA { 0x94A3B8 } else { cfg.color_low };
    let color_record = if cfg.color_high == 0xF43F5E { 0xE03131 } else { cfg.color_high };

    let mut buf = Vec::<u8>::with_capacity(n * 200 + 8192);
    svg_open(&mut buf, w, h);

    let half_arc = |buf: &mut Vec<u8>, r: f64| {
        let (x0, y0) = polar_point(cx, cy, start_angle, r);
        let (xm, ym) = polar_point(cx, cy, mid_angle, r);
        let (x1, y1) = polar_point(cx, cy, end_angle, r);
        push_b(buf, b"<path fill=\"none\" d=\"M");
        push_f2(buf, x0);
        push_b(buf, b",");
        push_f2(buf, y0);
        push_b(buf, b" A");
        push_f2(buf, r);
        push_b(buf, b",");
        push_f2(buf, r);
        push_b(buf, b" 0 0,1 ");
        push_f2(buf, xm);
        push_b(buf, b",");
        push_f2(buf, ym);
        push_b(buf, b" A");
        push_f2(buf, r);
        push_b(buf, b",");
        push_f2(buf, r);
        push_b(buf, b" 0 0,1 ");
        push_f2(buf, x1);
        push_b(buf, b",");
        push_f2(buf, y1);
        push_b(buf, b"\"/>");
    };

    push_b(&mut buf, b"<line x1=\"");
    push_f2(&mut buf, cx);
    push_b(&mut buf, b"\" y1=\"");
    push_f2(&mut buf, list_top);
    push_b(&mut buf, b"\" x2=\"");
    push_f2(&mut buf, cx);
    push_b(&mut buf, b"\" y2=\"");
    push_f2(&mut buf, list_bottom);
    push_b(&mut buf, b"\" stroke=\"#cbd5e1\" stroke-width=\"1\"/>");

    push_b(&mut buf, b"<g stroke=\"#e6eaf0\" stroke-width=\"1\">");
    for &r in &row_radii {
        half_arc(&mut buf, r);
    }
    push_b(&mut buf, b"</g>");

    let named_axis = !cfg.x_categories.is_empty();
    let tick_count = if named_axis { cfg.x_categories.len() } else { 31 };
    let tick_segs = tick_count.saturating_sub(1).max(1);
    let tick_angle = |k: usize| -> f64 { start_angle + sweep * k as f64 / tick_segs as f64 };
    let tick_text = |k: usize| -> String {
        if named_axis {
            cfg.x_categories[k].clone()
        } else {
            format!("{:.0}", xmin + (xmax - xmin) * k as f64 / tick_segs as f64)
        }
    };
    let label_every = if named_axis { 1 } else { (tick_segs / 10).max(1) };

    push_b(&mut buf, b"<g stroke=\"#eef1f6\" stroke-width=\"1\">");
    for k in 0..tick_count {
        let a = tick_angle(k);
        let (x0, y0) = polar_point(cx, cy, a, hub_r);
        let (x1, y1) = polar_point(cx, cy, a, outer_r);
        push_b(&mut buf, b"<line x1=\"");
        push_f2(&mut buf, x0);
        push_b(&mut buf, b"\" y1=\"");
        push_f2(&mut buf, y0);
        push_b(&mut buf, b"\" x2=\"");
        push_f2(&mut buf, x1);
        push_b(&mut buf, b"\" y2=\"");
        push_f2(&mut buf, y1);
        push_b(&mut buf, b"\"/>");
    }
    push_b(&mut buf, b"</g>");

    push_b(&mut buf, b"<g fill=\"none\" stroke=\"#94a3b8\" stroke-width=\"1.4\">");
    half_arc(&mut buf, outer_r);
    half_arc(&mut buf, hub_r);
    push_b(&mut buf, b"</g>");

    push_b(&mut buf, b"<g fill=\"#94a3b8\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"7\">");
    for k in (0..tick_count).step_by(label_every) {
        let a = tick_angle(k);
        let (tx, ty) = polar_point(cx, cy, a, outer_r + 9.0);
        let deg = a * 180.0 / PI;
        push_b(&mut buf, b"<text x=\"");
        push_f2(&mut buf, tx);
        push_b(&mut buf, b"\" y=\"");
        push_f2(&mut buf, ty);
        push_b(&mut buf, b"\" transform=\"rotate(");
        push_f2(&mut buf, deg);
        push_b(&mut buf, b" ");
        push_f2(&mut buf, tx);
        push_b(&mut buf, b" ");
        push_f2(&mut buf, ty);
        push_b(&mut buf, b")\">");
        escape_xml(&mut buf, &tick_text(k));
        push_b(&mut buf, b"</text>");
    }
    push_b(&mut buf, b"</g>");

    let mut slots: Vec<HoverSlot> = Vec::with_capacity(n);
    for i in 0..n {
        let ri = row_of(&cfg.categories[i]);
        let a = angle_of(cfg.x_values[i]);
        let r = row_radii[ri];
        let (px, py) = polar_point(cx, cy, a, r);
        let sn = (cfg.sizes[i].abs() - smin) / sr;
        let radius = cfg.min_size + sn * (cfg.max_size - cfg.min_size);
        let is_record = cfg.color_values.get(i).copied().unwrap_or(0.0) >= 0.5;
        let color = if is_record { color_record } else { color_normal };
        let hx = hex6(color);

        push_b(&mut buf, b"<circle data-idx=\"");
        push_i(&mut buf, i as i32);
        push_b(&mut buf, b"\" cx=\"");
        push_f2(&mut buf, px);
        push_b(&mut buf, b"\" cy=\"");
        push_f2(&mut buf, py);
        push_b(&mut buf, b"\" r=\"");
        push_f2(&mut buf, radius);
        push_b(&mut buf, b"\" fill=\"#");
        buf.extend_from_slice(&hx);
        push_b(&mut buf, b"\" fill-opacity=\"");
        push_f2(&mut buf, if is_record { 0.92 } else { 0.62 });
        push_b(&mut buf, b"\"");
        if is_record {
            push_b(&mut buf, b" stroke=\"#");
            buf.extend_from_slice(&hex6(color_record));
            push_b(&mut buf, b"\" stroke-width=\"1.4\" stroke-opacity=\"0.9\"");
        }
        push_b(&mut buf, b"/>");

        let label = cfg.labels.get(i).map(|s| s.as_str()).unwrap_or("");
        let mut slot = HoverSlot::new(if label.is_empty() { cfg.categories[i].clone() } else { label.to_string() })
            .kv("Row", cfg.categories[i].clone())
            .kv("Position", format!("{:.0}", cfg.x_values[i]))
            .kv("Size", format!("{:.1}", cfg.sizes[i]));
        if is_record {
            slot = slot.kv("Record", "yes".to_string());
        }
        slots.push(slot);
    }

    for ri in 0..n_rows {
        let ly = label_ys[ri];
        push_b(&mut buf, b"<line x1=\"");
        push_f2(&mut buf, label_w - 6.0);
        push_b(&mut buf, b"\" y1=\"");
        push_f2(&mut buf, ly);
        push_b(&mut buf, b"\" x2=\"");
        push_f2(&mut buf, cx - 2.0);
        push_b(&mut buf, b"\" y2=\"");
        push_f2(&mut buf, ly);
        push_b(&mut buf, b"\" stroke=\"#dbe1ea\" stroke-width=\"1\"/>");
        push_b(&mut buf, b"<circle cx=\"");
        push_f2(&mut buf, cx);
        push_b(&mut buf, b"\" cy=\"");
        push_f2(&mut buf, ly);
        push_b(&mut buf, b"\" r=\"1.4\" fill=\"#94a3b8\"/>");
        push_b(&mut buf, b"<text x=\"");
        push_f2(&mut buf, label_w - 10.0);
        push_b(&mut buf, b"\" y=\"");
        push_f2(&mut buf, ly + 3.0);
        push_b(&mut buf, b"\" text-anchor=\"end\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"9\" fill=\"#64748b\">");
        escape_xml(&mut buf, &rows[ri]);
        push_b(&mut buf, b"</text>");
    }

    if !cfg.title.is_empty() {
        push_b(&mut buf, b"<text x=\"");
        push_f2(&mut buf, cx);
        push_b(&mut buf, b"\" y=\"22\" text-anchor=\"middle\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"12\" font-weight=\"700\" fill=\"#1e293b\" letter-spacing=\"2\">");
        escape_xml(&mut buf, cfg.title);
        push_b(&mut buf, b"</text>");
    }

    let leg_x = w - 150;
    let leg_y = h - 84;
    push_b(&mut buf, b"<text x=\"");
    push_i(&mut buf, leg_x);
    push_b(&mut buf, b"\" y=\"");
    push_i(&mut buf, leg_y - 14);
    push_b(&mut buf, b"\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"8.5\" font-weight=\"700\" fill=\"#475569\">SIZE</text>");
    let sizes_legend = [cfg.min_size, (cfg.min_size + cfg.max_size) / 2.0, cfg.max_size];
    for (k, &r) in sizes_legend.iter().enumerate() {
        let sy = leg_y - r as i32;
        push_b(&mut buf, b"<circle cx=\"");
        push_i(&mut buf, leg_x + k as i32 * 34);
        push_b(&mut buf, b"\" cy=\"");
        push_i(&mut buf, sy);
        push_b(&mut buf, b"\" r=\"");
        push_f2(&mut buf, r);
        push_b(&mut buf, b"\" fill=\"none\" stroke=\"#94a3b8\" stroke-opacity=\"0.7\" stroke-width=\"1\"/>");
        push_b(&mut buf, b"<text x=\"");
        push_i(&mut buf, leg_x + k as i32 * 34);
        push_b(&mut buf, b"\" y=\"");
        push_i(&mut buf, leg_y + 12);
        push_b(&mut buf, b"\" text-anchor=\"middle\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"7\" fill=\"#94a3b8\">");
        let s = format!("{:.0}", r);
        buf.extend_from_slice(s.as_bytes());
        push_b(&mut buf, b"</text>");
    }
    push_b(&mut buf, b"<circle cx=\"");
    push_i(&mut buf, leg_x);
    push_b(&mut buf, b"\" cy=\"");
    push_i(&mut buf, leg_y + 26);
    push_b(&mut buf, b"\" r=\"5\" fill=\"#");
    buf.extend_from_slice(&hex6(color_normal));
    push_b(&mut buf, b"\" fill-opacity=\"0.62\"/>");
    push_b(&mut buf, b"<text x=\"");
    push_i(&mut buf, leg_x + 10);
    push_b(&mut buf, b"\" y=\"");
    push_i(&mut buf, leg_y + 30);
    push_b(&mut buf, b"\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"8\" fill=\"#475569\">Normal</text>");
    push_b(&mut buf, b"<circle cx=\"");
    push_i(&mut buf, leg_x + 70);
    push_b(&mut buf, b"\" cy=\"");
    push_i(&mut buf, leg_y + 26);
    push_b(&mut buf, b"\" r=\"5\" fill=\"#");
    buf.extend_from_slice(&hex6(color_record));
    push_b(&mut buf, b"\" fill-opacity=\"0.92\" stroke=\"#");
    buf.extend_from_slice(&hex6(color_record));
    push_b(&mut buf, b"\" stroke-width=\"1.4\"/>");
    push_b(&mut buf, b"<text x=\"");
    push_i(&mut buf, leg_x + 80);
    push_b(&mut buf, b"\" y=\"");
    push_i(&mut buf, leg_y + 30);
    push_b(&mut buf, b"\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"8\" fill=\"#475569\">Record</text>");

    push_b(&mut buf, b"</svg>");
    let svg = unsafe { String::from_utf8_unchecked(buf) };
    build_chart_html(cfg.title, &svg, &slots_to_json(&slots))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::plot::statistical::bubble::config::BubbleConfig;

    fn cfg<'a>(
        x: &'a [f64],
        sizes: &'a [f64],
        cats: &'a [String],
        colv: &'a [f64],
    ) -> BubbleConfig<'a> {
        BubbleConfig {
            title: "Test",
            x_values: x,
            sizes,
            categories: cats,
            color_values: colv,
            width: 900,
            height: 700,
            ..BubbleConfig::default()
        }
    }

    fn synth(n: usize, n_rows: usize) -> (Vec<f64>, Vec<f64>, Vec<String>, Vec<f64>) {
        let x: Vec<f64> = (0..n).map(|i| i as f64).collect();
        let sizes: Vec<f64> = (0..n).map(|i| 5.0 + (i % 7) as f64 * 4.0).collect();
        let cats: Vec<String> = (0..n).map(|i| format!("Row {}", i % n_rows)).collect();
        let colv: Vec<f64> = (0..n).map(|i| if i % 11 == 0 { 1.0 } else { 0.0 }).collect();
        (x, sizes, cats, colv)
    }

    #[test]
    fn renders_one_bubble_per_point_across_the_row_rings() {
        let (x, sizes, cats, colv) = synth(40, 6);
        let html = render(&cfg(&x, &sizes, &cats, &colv));
        assert!(!html.is_empty());
        assert_eq!(html.matches("<circle data-idx=").count(), 40);
        assert!(html.contains("class=\"sp-bg\""));
        assert!(html.contains("viewBox=\"0 0 900 700\""));
    }

    #[test]
    fn record_points_get_the_record_color_and_a_stroke() {
        let (x, sizes, cats, colv) = synth(20, 4);
        let html = render(&cfg(&x, &sizes, &cats, &colv));
        assert!(html.contains("Record</text>"));
        assert!(html.matches("stroke-width=\"1.4\" stroke-opacity=\"0.9\"").count() > 0);
    }

    #[test]
    fn each_distinct_row_gets_its_own_labeled_ring() {
        let (x, sizes, cats, colv) = synth(30, 5);
        let html = render(&cfg(&x, &sizes, &cats, &colv));
        for r in 0..5 {
            assert!(html.contains(&format!(">Row {r}<")));
        }
    }

    #[test]
    fn empty_input_returns_empty_string() {
        let x: Vec<f64> = vec![];
        let sizes: Vec<f64> = vec![];
        let cats: Vec<String> = vec![];
        let colv: Vec<f64> = vec![];
        assert!(render(&cfg(&x, &sizes, &cats, &colv)).is_empty());
    }

    #[test]
    fn named_x_categories_replace_numeric_ticks_with_one_spoke_per_label() {
        let (x, sizes, cats, colv) = synth(30, 5);
        let axis: Vec<String> = ["pont fer", "pont bois", "pont pierre", "cheminee bois", "cheminee pierre"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let html = render(&BubbleConfig {
            x_categories: &axis,
            ..cfg(&x, &sizes, &cats, &colv)
        });
        for label in &axis {
            assert!(html.contains(&format!(">{label}<")));
        }
        assert!(!html.contains(">31<"));
    }

    #[test]
    fn without_x_categories_the_axis_falls_back_to_numeric_ticks() {
        let (x, sizes, cats, colv) = synth(30, 5);
        let html = render(&cfg(&x, &sizes, &cats, &colv));
        assert!(html.contains(">0<"));
        assert!(!html.contains("pont"));
    }

    #[test]
    fn perf_rendering_a_large_radial_row_chart_stays_fast() {
        let (x, sizes, cats, colv) = synth(1200, 60);
        let start = std::time::Instant::now();
        let html = render(&cfg(&x, &sizes, &cats, &colv));
        let elapsed = start.elapsed();
        assert!(elapsed.as_millis() < 200, "rendering took too long: {elapsed:?}");
        assert!(!html.is_empty());
    }
}
