use super::config::BubbleConfig;
use crate::html::hover::{build_chart_html, slots_to_json, HoverSlot};
use crate::plot::statistical::common::{escape_xml, hex6, polar_point, push_b, push_f2, push_i, svg_open};
use std::f64::consts::PI;

#[crate::chart_demo(
    "labels=[\"Melbourne GP #1\",\"Suzuka GP #2\",\"Baku GP #3\",\"Barcelona GP #4\",\"Yas Marina GP #5\",\"Jeddah GP #6\",\"Monaco GP #7\",\"Interlagos GP #8\",\"Zandvoort GP #9\",\"Montreal GP #10\",\"Singapore GP #11\",\"Silverstone GP #12\",\"Silverstone GP #13\",\"Budapest GP #14\",\"Monaco GP #15\",\"Austin GP #16\",\"Zandvoort GP #17\",\"Mexico City GP #18\",\"Sochi GP #19\",\"Monza GP #20\",\"Jeddah GP #21\",\"Barcelona GP #22\",\"Monaco GP #23\",\"Shanghai GP #24\",\"Monza GP #25\",\"Nurburgring GP #26\",\"Sochi GP #27\",\"Monza GP #28\",\"Sochi GP #29\",\"Yas Marina GP #30\",\"Spa GP #31\",\"Baku GP #32\",\"Zolder GP #33\",\"Imola GP #34\",\"Montreal GP #35\",\"Monaco GP #36\",\"Yas Marina GP #37\",\"Zandvoort GP #38\",\"Mexico City GP #39\",\"Mexico City GP #40\",\"Shanghai GP #41\",\"Singapore GP #42\",\"Mexico City GP #43\",\"Yas Marina GP #44\",\"Silverstone GP #45\",\"Shanghai GP #46\",\"Sepang GP #47\",\"Sepang GP #48\",\"Zandvoort GP #49\",\"Shanghai GP #50\",\"Montreal GP #51\",\"Sochi GP #52\",\"Shanghai GP #53\",\"Monaco GP #54\",\"Shanghai GP #55\",\"Mexico City GP #56\",\"Silverstone GP #57\",\"Sepang GP #58\",\"Sepang GP #59\",\"Suzuka GP #60\",\"Hockenheim GP #61\",\"Silverstone GP #62\",\"Melbourne GP #63\",\"Jeddah GP #64\",\"Interlagos GP #65\",\"Sochi GP #66\",\"Spa GP #67\",\"Budapest GP #68\",\"Nurburgring GP #69\",\"Nurburgring GP #70\",\"Budapest GP #71\",\"Melbourne GP #72\",\"Silverstone GP #73\",\"Melbourne GP #74\",\"Baku GP #75\",\"Suzuka GP #76\",\"Nurburgring GP #77\",\"Melbourne GP #78\",\"Montreal GP #79\",\"Zolder GP #80\",\"Montreal GP #81\",\"Imola GP #82\",\"Sochi GP #83\",\"Interlagos GP #84\",\"Suzuka GP #85\",\"Zolder GP #86\",\"Montreal GP #87\",\"Austin GP #88\",\"Interlagos GP #89\",\"Interlagos GP #90\",\"Zandvoort GP #91\",\"Silverstone GP #92\",\"Jeddah GP #93\",\"Silverstone GP #94\",\"Budapest GP #95\",\"Montreal GP #96\",\"Nurburgring GP #97\",\"Interlagos GP #98\",\"Sochi GP #99\",\"Interlagos GP #100\",\"Spa GP #101\",\"Montreal GP #102\",\"Budapest GP #103\",\"Suzuka GP #104\",\"Shanghai GP #105\",\"Zolder GP #106\",\"Monza GP #107\",\"Sepang GP #108\",\"Spa GP #109\",\"Monaco GP #110\",\"Monaco GP #111\",\"Nurburgring GP #112\",\"Budapest GP #113\",\"Spa GP #114\",\"Melbourne GP #115\",\"Nurburgring GP #116\",\"Monaco GP #117\",\"Jeddah GP #118\",\"Interlagos GP #119\",\"Austin GP #120\",\"Suzuka GP #121\",\"Monaco GP #122\",\"Monza GP #123\",\"Spa GP #124\",\"Baku GP #125\",\"Sochi GP #126\",\"Monza GP #127\",\"Shanghai GP #128\",\"Zolder GP #129\",\"Spa GP #130\",\"Suzuka GP #131\",\"Zandvoort GP #132\",\"Sochi GP #133\",\"Imola GP #134\",\"Melbourne GP #135\",\"Nurburgring GP #136\",\"Silverstone GP #137\",\"Mexico City GP #138\",\"Baku GP #139\",\"Imola GP #140\",\"Monaco GP #141\",\"Baku GP #142\",\"Interlagos GP #143\",\"Suzuka GP #144\",\"Nurburgring GP #145\",\"Shanghai GP #146\",\"Spa GP #147\",\"Shanghai GP #148\",\"Suzuka GP #149\",\"Monza GP #150\",\"Zandvoort GP #151\",\"Interlagos GP #152\",\"Monaco GP #153\",\"Yas Marina GP #154\",\"Zolder GP #155\",\"Mexico City GP #156\",\"Zolder GP #157\",\"Spa GP #158\",\"Montreal GP #159\",\"Suzuka GP #160\",\"Monza GP #161\",\"Hockenheim GP #162\",\"Monaco GP #163\",\"Barcelona GP #164\",\"Spa GP #165\",\"Melbourne GP #166\",\"Silverstone GP #167\",\"Budapest GP #168\",\"Singapore GP #169\",\"Imola GP #170\",\"Spa GP #171\",\"Mexico City GP #172\",\"Spa GP #173\",\"Monza GP #174\",\"Suzuka GP #175\",\"Monza GP #176\",\"Nurburgring GP #177\",\"Hockenheim GP #178\",\"Jeddah GP #179\",\"Budapest GP #180\",\"Sepang GP #181\",\"Shanghai GP #182\",\"Nurburgring GP #183\",\"Monaco GP #184\",\"Interlagos GP #185\",\"Budapest GP #186\",\"Spa GP #187\",\"Yas Marina GP #188\",\"Imola GP #189\",\"Silverstone GP #190\",\"Austin GP #191\",\"Monaco GP #192\",\"Yas Marina GP #193\",\"Hockenheim GP #194\",\"Imola GP #195\",\"Yas Marina GP #196\",\"Melbourne GP #197\",\"Nurburgring GP #198\",\"Silverstone GP #199\",\"Baku GP #200\",\"Interlagos GP #201\",\"Zandvoort GP #202\",\"Zandvoort GP #203\",\"Sochi GP #204\",\"Nurburgring GP #205\",\"Montreal GP #206\",\"Spa GP #207\",\"Suzuka GP #208\",\"Shanghai GP #209\",\"Sochi GP #210\",\"Melbourne GP #211\",\"Hockenheim GP #212\",\"Nurburgring GP #213\",\"Nurburgring GP #214\",\"Yas Marina GP #215\",\"Barcelona GP #216\",\"Melbourne GP #217\",\"Interlagos GP #218\",\"Zandvoort GP #219\",\"Singapore GP #220\",\"Interlagos GP #221\",\"Austin GP #222\",\"Jeddah GP #223\",\"Austin GP #224\",\"Montreal GP #225\",\"Zolder GP #226\",\"Silverstone GP #227\",\"Sepang GP #228\",\"Melbourne GP #229\",\"Shanghai GP #230\",\"Nurburgring GP #231\",\"Hockenheim GP #232\",\"Jeddah GP #233\",\"Mexico City GP #234\",\"Budapest GP #235\",\"Austin GP #236\",\"Barcelona GP #237\",\"Interlagos GP #238\",\"Monaco GP #239\",\"Budapest GP #240\",\"Monaco GP #241\",\"Shanghai GP #242\",\"Montreal GP #243\",\"Hockenheim GP #244\",\"Yas Marina GP #245\",\"Sochi GP #246\",\"Monaco GP #247\",\"Monaco GP #248\",\"Mexico City GP #249\",\"Sochi GP #250\",\"Yas Marina GP #251\",\"Spa GP #252\",\"Suzuka GP #253\",\"Shanghai GP #254\",\"Nurburgring GP #255\",\"Spa GP #256\",\"Shanghai GP #257\",\"Yas Marina GP #258\",\"Singapore GP #259\",\"Montreal GP #260\",\"Jeddah GP #261\",\"Montreal GP #262\",\"Budapest GP #263\",\"Baku GP #264\",\"Nurburgring GP #265\",\"Monza GP #266\",\"Montreal GP #267\",\"Nurburgring GP #268\",\"Zandvoort GP #269\",\"Zandvoort GP #270\",\"Imola GP #271\",\"Monza GP #272\",\"Imola GP #273\",\"Interlagos GP #274\",\"Montreal GP #275\",\"Mexico City GP #276\",\"Jeddah GP #277\",\"Silverstone GP #278\",\"Nurburgring GP #279\",\"Jeddah GP #280\",\"Austin GP #281\",\"Mexico City GP #282\",\"Zolder GP #283\",\"Interlagos GP #284\",\"Shanghai GP #285\",\"Imola GP #286\",\"Jeddah GP #287\",\"Silverstone GP #288\",\"Melbourne GP #289\",\"Jeddah GP #290\",\"Imola GP #291\",\"Nurburgring GP #292\",\"Montreal GP #293\",\"Monza GP #294\",\"Nurburgring GP #295\",\"Budapest GP #296\",\"Budapest GP #297\",\"Austin GP #298\",\"Singapore GP #299\",\"Nurburgring GP #300\",\"Sochi GP #301\",\"Silverstone GP #302\",\"Silverstone GP #303\",\"Sepang GP #304\",\"Barcelona GP #305\",\"Yas Marina GP #306\",\"Monaco GP #307\",\"Imola GP #308\",\"Nurburgring GP #309\",\"Budapest GP #310\",\"Zandvoort GP #311\",\"Suzuka GP #312\",\"Monza GP #313\",\"Yas Marina GP #314\",\"Mexico City GP #315\",\"Sochi GP #316\",\"Sepang GP #317\",\"Baku GP #318\",\"Monaco GP #319\",\"Austin GP #320\",\"Sochi GP #321\",\"Silverstone GP #322\",\"Singapore GP #323\",\"Suzuka GP #324\",\"Monza GP #325\",\"Monza GP #326\",\"Baku GP #327\",\"Sepang GP #328\",\"Austin GP #329\",\"Silverstone GP #330\",\"Jeddah GP #331\",\"Suzuka GP #332\",\"Suzuka GP #333\",\"Monza GP #334\",\"Sepang GP #335\",\"Montreal GP #336\",\"Budapest GP #337\",\"Interlagos GP #338\",\"Jeddah GP #339\",\"Nurburgring GP #340\",\"Interlagos GP #341\",\"Zandvoort GP #342\",\"Sepang GP #343\",\"Spa GP #344\",\"Spa GP #345\",\"Budapest GP #346\",\"Nurburgring GP #347\",\"Interlagos GP #348\",\"Spa GP #349\",\"Nurburgring GP #350\",\"Interlagos GP #351\",\"Sochi GP #352\",\"Spa GP #353\",\"Silverstone GP #354\",\"Spa GP #355\",\"Yas Marina GP #356\",\"Mexico City GP #357\",\"Monza GP #358\",\"Imola GP #359\",\"Zolder GP #360\",\"Montreal GP #361\",\"Sepang GP #362\",\"Sochi GP #363\",\"Monza GP #364\",\"Imola GP #365\",\"Montreal GP #366\",\"Interlagos GP #367\",\"Melbourne GP #368\",\"Shanghai GP #369\",\"Zolder GP #370\",\"Monza GP #371\",\"Baku GP #372\",\"Spa GP #373\",\"Zandvoort GP #374\",\"Spa GP #375\",\"Melbourne GP #376\",\"Baku GP #377\",\"Zandvoort GP #378\",\"Nurburgring GP #379\",\"Spa GP #380\"], x_values=[1.0,2.0,3.0,4.0,5.0,6.0,7.0,8.0,9.0,10.0,11.0,12.0,13.0,14.0,15.0,16.0,17.0,18.0,19.0,20.0,21.0,22.0,23.0,24.0,25.0,26.0,27.0,28.0,29.0,30.0,31.0,32.0,33.0,34.0,35.0,36.0,37.0,38.0,39.0,40.0,41.0,42.0,43.0,44.0,45.0,46.0,47.0,48.0,49.0,50.0,51.0,52.0,53.0,54.0,55.0,56.0,57.0,58.0,59.0,60.0,61.0,62.0,63.0,64.0,65.0,66.0,67.0,68.0,69.0,70.0,71.0,72.0,73.0,74.0,75.0,76.0,77.0,78.0,79.0,80.0,81.0,82.0,83.0,84.0,85.0,86.0,87.0,88.0,89.0,90.0,91.0,92.0,93.0,94.0,95.0,96.0,97.0,98.0,99.0,100.0,101.0,102.0,103.0,104.0,105.0,106.0,107.0,108.0,109.0,110.0,111.0,112.0,113.0,114.0,115.0,116.0,117.0,118.0,119.0,120.0,121.0,122.0,123.0,124.0,125.0,126.0,127.0,128.0,129.0,130.0,131.0,132.0,133.0,134.0,135.0,136.0,137.0,138.0,139.0,140.0,141.0,142.0,143.0,144.0,145.0,146.0,147.0,148.0,149.0,150.0,151.0,152.0,153.0,154.0,155.0,156.0,157.0,158.0,159.0,160.0,161.0,162.0,163.0,164.0,165.0,166.0,167.0,168.0,169.0,170.0,171.0,172.0,173.0,174.0,175.0,176.0,177.0,178.0,179.0,180.0,181.0,182.0,183.0,184.0,185.0,186.0,187.0,188.0,189.0,190.0,191.0,192.0,193.0,194.0,195.0,196.0,197.0,198.0,199.0,200.0,201.0,202.0,203.0,204.0,205.0,206.0,207.0,208.0,209.0,210.0,211.0,212.0,213.0,214.0,215.0,216.0,217.0,218.0,219.0,220.0,221.0,222.0,223.0,224.0,225.0,226.0,227.0,228.0,229.0,230.0,231.0,232.0,233.0,234.0,235.0,236.0,237.0,238.0,239.0,240.0,241.0,242.0,243.0,244.0,245.0,246.0,247.0,248.0,249.0,250.0,251.0,252.0,253.0,254.0,255.0,256.0,257.0,258.0,259.0,260.0,261.0,262.0,263.0,264.0,265.0,266.0,267.0,268.0,269.0,270.0,271.0,272.0,273.0,274.0,275.0,276.0,277.0,278.0,279.0,280.0,281.0,282.0,283.0,284.0,285.0,286.0,287.0,288.0,289.0,290.0,291.0,292.0,293.0,294.0,295.0,296.0,297.0,298.0,299.0,300.0,301.0,302.0,303.0,304.0,305.0,306.0,307.0,308.0,309.0,310.0,311.0,312.0,313.0,314.0,315.0,316.0,317.0,318.0,319.0,320.0,321.0,322.0,323.0,324.0,325.0,326.0,327.0,328.0,329.0,330.0,331.0,332.0,333.0,334.0,335.0,336.0,337.0,338.0,339.0,340.0,341.0,342.0,343.0,344.0,345.0,346.0,347.0,348.0,349.0,350.0,351.0,352.0,353.0,354.0,355.0,356.0,357.0,358.0,359.0,360.0,361.0,362.0,363.0,364.0,365.0,366.0,367.0,368.0,369.0,370.0,371.0,372.0,373.0,374.0,375.0,376.0,377.0,378.0,379.0,380.0], sizes=[40.2,15.3,17.8,31.0,31.5,18.2,13.0,37.5,18.5,18.7,9.9,33.5,34.8,8.3,40.5,39.6,20.1,11.7,41.2,19.6,19.6,18.6,10.7,41.9,41.5,16.1,23.5,34.9,12.4,29.4,22.8,18.0,37.0,24.6,17.9,20.8,29.3,11.0,37.5,34.3,33.8,17.5,31.5,30.1,26.6,23.7,35.1,33.1,36.7,41.2,26.0,39.9,32.5,34.5,22.3,29.7,13.4,15.4,9.4,9.8,14.2,23.8,28.1,8.0,21.9,36.6,26.5,40.6,35.6,12.8,40.5,19.9,11.7,12.8,20.8,17.9,40.7,16.8,22.1,24.7,12.1,18.0,8.4,39.8,17.1,18.0,29.9,12.0,26.1,26.0,33.3,16.6,38.2,34.5,32.1,39.0,37.0,36.2,31.6,34.5,24.3,19.2,35.6,35.9,8.9,41.9,36.6,40.4,17.9,28.8,29.5,10.4,33.2,22.7,24.6,9.9,29.5,26.9,26.8,25.6,20.6,32.3,30.7,28.2,24.3,9.0,13.1,15.6,30.6,29.2,19.3,21.1,30.0,37.0,27.5,30.8,38.2,8.0,29.1,11.4,20.7,13.2,13.5,40.9,39.9,24.0,30.7,32.8,8.8,21.6,18.6,32.9,16.2,30.0,8.2,35.7,25.6,10.4,39.8,9.7,16.7,41.6,25.3,10.9,26.4,37.1,26.3,20.8,39.0,36.1,18.6,35.9,10.8,22.6,10.1,14.8,11.4,33.6,30.2,23.0,40.5,17.1,12.4,16.9,10.2,8.9,20.1,28.8,20.3,27.9,11.2,32.1,35.0,39.1,23.0,26.1,11.5,9.8,27.6,21.4,14.8,33.5,16.4,36.9,12.9,9.3,32.4,35.1,37.3,35.2,14.3,8.9,24.2,41.4,31.6,35.2,31.1,10.6,27.7,18.4,32.3,30.8,31.4,28.8,38.5,26.5,21.3,38.6,21.1,26.8,38.2,17.1,13.7,15.6,36.2,22.2,38.8,28.6,9.0,28.5,28.2,32.8,36.4,36.9,15.8,24.2,13.3,28.9,33.0,32.2,29.3,11.5,20.7,33.3,26.5,28.3,24.8,10.5,18.1,20.0,21.0,31.7,18.1,25.7,16.1,17.2,41.5,39.8,38.7,8.6,21.0,15.4,24.0,40.5,9.8,22.8,29.9,8.8,37.7,20.9,38.3,22.4,15.4,24.0,12.2,10.5,38.1,36.2,12.6,30.6,17.1,34.4,41.1,11.8,20.5,40.8,16.9,12.5,8.3,25.1,9.9,21.1,18.1,39.4,32.2,41.6,39.7,33.0,39.4,36.2,24.8,23.4,19.6,17.0,33.2,29.3,16.4,22.9,12.7,28.6,24.8,28.0,28.8,40.3,31.3,23.6,32.5,36.7,16.8,33.8,23.8,17.0,11.0,22.5,13.9,31.4,39.0,24.8,26.9,34.7,11.7,38.2,29.8,34.2,29.5,9.7,29.6,21.9,10.2,37.6,14.1,38.5,22.1,27.2,25.4,23.8,11.9,41.6,17.3,22.5,39.2,39.2,26.0,39.9,39.2,17.7,24.0,12.6,32.8,12.5,23.2,13.5,27.7,9.8,34.0,29.7,39.3,37.8,22.1,30.3], categories=[\"Melbourne\",\"Suzuka\",\"Baku\",\"Barcelona\",\"Yas Marina\",\"Jeddah\",\"Monaco\",\"Interlagos\",\"Zandvoort\",\"Montreal\",\"Singapore\",\"Silverstone\",\"Silverstone\",\"Budapest\",\"Monaco\",\"Austin\",\"Zandvoort\",\"Mexico City\",\"Sochi\",\"Monza\",\"Jeddah\",\"Barcelona\",\"Monaco\",\"Shanghai\",\"Monza\",\"Nurburgring\",\"Sochi\",\"Monza\",\"Sochi\",\"Yas Marina\",\"Spa\",\"Baku\",\"Zolder\",\"Imola\",\"Montreal\",\"Monaco\",\"Yas Marina\",\"Zandvoort\",\"Mexico City\",\"Mexico City\",\"Shanghai\",\"Singapore\",\"Mexico City\",\"Yas Marina\",\"Silverstone\",\"Shanghai\",\"Sepang\",\"Sepang\",\"Zandvoort\",\"Shanghai\",\"Montreal\",\"Sochi\",\"Shanghai\",\"Monaco\",\"Shanghai\",\"Mexico City\",\"Silverstone\",\"Sepang\",\"Sepang\",\"Suzuka\",\"Hockenheim\",\"Silverstone\",\"Melbourne\",\"Jeddah\",\"Interlagos\",\"Sochi\",\"Spa\",\"Budapest\",\"Nurburgring\",\"Nurburgring\",\"Budapest\",\"Melbourne\",\"Silverstone\",\"Melbourne\",\"Baku\",\"Suzuka\",\"Nurburgring\",\"Melbourne\",\"Montreal\",\"Zolder\",\"Montreal\",\"Imola\",\"Sochi\",\"Interlagos\",\"Suzuka\",\"Zolder\",\"Montreal\",\"Austin\",\"Interlagos\",\"Interlagos\",\"Zandvoort\",\"Silverstone\",\"Jeddah\",\"Silverstone\",\"Budapest\",\"Montreal\",\"Nurburgring\",\"Interlagos\",\"Sochi\",\"Interlagos\",\"Spa\",\"Montreal\",\"Budapest\",\"Suzuka\",\"Shanghai\",\"Zolder\",\"Monza\",\"Sepang\",\"Spa\",\"Monaco\",\"Monaco\",\"Nurburgring\",\"Budapest\",\"Spa\",\"Melbourne\",\"Nurburgring\",\"Monaco\",\"Jeddah\",\"Interlagos\",\"Austin\",\"Suzuka\",\"Monaco\",\"Monza\",\"Spa\",\"Baku\",\"Sochi\",\"Monza\",\"Shanghai\",\"Zolder\",\"Spa\",\"Suzuka\",\"Zandvoort\",\"Sochi\",\"Imola\",\"Melbourne\",\"Nurburgring\",\"Silverstone\",\"Mexico City\",\"Baku\",\"Imola\",\"Monaco\",\"Baku\",\"Interlagos\",\"Suzuka\",\"Nurburgring\",\"Shanghai\",\"Spa\",\"Shanghai\",\"Suzuka\",\"Monza\",\"Zandvoort\",\"Interlagos\",\"Monaco\",\"Yas Marina\",\"Zolder\",\"Mexico City\",\"Zolder\",\"Spa\",\"Montreal\",\"Suzuka\",\"Monza\",\"Hockenheim\",\"Monaco\",\"Barcelona\",\"Spa\",\"Melbourne\",\"Silverstone\",\"Budapest\",\"Singapore\",\"Imola\",\"Spa\",\"Mexico City\",\"Spa\",\"Monza\",\"Suzuka\",\"Monza\",\"Nurburgring\",\"Hockenheim\",\"Jeddah\",\"Budapest\",\"Sepang\",\"Shanghai\",\"Nurburgring\",\"Monaco\",\"Interlagos\",\"Budapest\",\"Spa\",\"Yas Marina\",\"Imola\",\"Silverstone\",\"Austin\",\"Monaco\",\"Yas Marina\",\"Hockenheim\",\"Imola\",\"Yas Marina\",\"Melbourne\",\"Nurburgring\",\"Silverstone\",\"Baku\",\"Interlagos\",\"Zandvoort\",\"Zandvoort\",\"Sochi\",\"Nurburgring\",\"Montreal\",\"Spa\",\"Suzuka\",\"Shanghai\",\"Sochi\",\"Melbourne\",\"Hockenheim\",\"Nurburgring\",\"Nurburgring\",\"Yas Marina\",\"Barcelona\",\"Melbourne\",\"Interlagos\",\"Zandvoort\",\"Singapore\",\"Interlagos\",\"Austin\",\"Jeddah\",\"Austin\",\"Montreal\",\"Zolder\",\"Silverstone\",\"Sepang\",\"Melbourne\",\"Shanghai\",\"Nurburgring\",\"Hockenheim\",\"Jeddah\",\"Mexico City\",\"Budapest\",\"Austin\",\"Barcelona\",\"Interlagos\",\"Monaco\",\"Budapest\",\"Monaco\",\"Shanghai\",\"Montreal\",\"Hockenheim\",\"Yas Marina\",\"Sochi\",\"Monaco\",\"Monaco\",\"Mexico City\",\"Sochi\",\"Yas Marina\",\"Spa\",\"Suzuka\",\"Shanghai\",\"Nurburgring\",\"Spa\",\"Shanghai\",\"Yas Marina\",\"Singapore\",\"Montreal\",\"Jeddah\",\"Montreal\",\"Budapest\",\"Baku\",\"Nurburgring\",\"Monza\",\"Montreal\",\"Nurburgring\",\"Zandvoort\",\"Zandvoort\",\"Imola\",\"Monza\",\"Imola\",\"Interlagos\",\"Montreal\",\"Mexico City\",\"Jeddah\",\"Silverstone\",\"Nurburgring\",\"Jeddah\",\"Austin\",\"Mexico City\",\"Zolder\",\"Interlagos\",\"Shanghai\",\"Imola\",\"Jeddah\",\"Silverstone\",\"Melbourne\",\"Jeddah\",\"Imola\",\"Nurburgring\",\"Montreal\",\"Monza\",\"Nurburgring\",\"Budapest\",\"Budapest\",\"Austin\",\"Singapore\",\"Nurburgring\",\"Sochi\",\"Silverstone\",\"Silverstone\",\"Sepang\",\"Barcelona\",\"Yas Marina\",\"Monaco\",\"Imola\",\"Nurburgring\",\"Budapest\",\"Zandvoort\",\"Suzuka\",\"Monza\",\"Yas Marina\",\"Mexico City\",\"Sochi\",\"Sepang\",\"Baku\",\"Monaco\",\"Austin\",\"Sochi\",\"Silverstone\",\"Singapore\",\"Suzuka\",\"Monza\",\"Monza\",\"Baku\",\"Sepang\",\"Austin\",\"Silverstone\",\"Jeddah\",\"Suzuka\",\"Suzuka\",\"Monza\",\"Sepang\",\"Montreal\",\"Budapest\",\"Interlagos\",\"Jeddah\",\"Nurburgring\",\"Interlagos\",\"Zandvoort\",\"Sepang\",\"Spa\",\"Spa\",\"Budapest\",\"Nurburgring\",\"Interlagos\",\"Spa\",\"Nurburgring\",\"Interlagos\",\"Sochi\",\"Spa\",\"Silverstone\",\"Spa\",\"Yas Marina\",\"Mexico City\",\"Monza\",\"Imola\",\"Zolder\",\"Montreal\",\"Sepang\",\"Sochi\",\"Monza\",\"Imola\",\"Montreal\",\"Interlagos\",\"Melbourne\",\"Shanghai\",\"Zolder\",\"Monza\",\"Baku\",\"Spa\",\"Zandvoort\",\"Spa\",\"Melbourne\",\"Baku\",\"Zandvoort\",\"Nurburgring\",\"Spa\"], color_values=[1.0,0.0,0.0,0.0,0.0,0.0,0.0,1.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,1.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,1.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,1.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,1.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,1.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,1.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,1.0,0.0,0.0,0.0,0.0,0.0,1.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,1.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,1.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,1.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,1.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0], variant=\"radial_rows\", width=1050, height=950, min_size=2.5, max_size=11"
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
    let cx = label_w + (w as f64 - label_w) / 2.0;
    let cy = h as f64 / 2.0 + 6.0;
    let side = (w as f64 - label_w).min(h as f64);
    let inner_r = side * 0.10;
    let outer_r = side * 0.46;
    let sweep = 2.0 * PI * 0.80;
    let gap_half = (2.0 * PI - sweep) / 2.0;
    let start_angle = PI + gap_half;

    let angle_of = |x: f64| -> f64 { start_angle + ((x - xmin) / xr) * sweep };
    let row_r = |ri: usize| -> f64 {
        if n_rows <= 1 {
            (inner_r + outer_r) / 2.0
        } else {
            inner_r + (ri as f64 / (n_rows - 1) as f64) * (outer_r - inner_r)
        }
    };

    let color_normal = if cfg.color_low == 0x636EFA { 0x94A3B8 } else { cfg.color_low };
    let color_record = if cfg.color_high == 0xF43F5E { 0xE03131 } else { cfg.color_high };

    let mut buf = Vec::<u8>::with_capacity(n * 200 + 8192);
    svg_open(&mut buf, w, h);

    push_b(&mut buf, b"<g stroke=\"#e2e8f0\" stroke-width=\"1\">");
    for ri in 0..n_rows {
        let r = row_r(ri);
        let a0 = start_angle;
        let a1 = start_angle + sweep;
        let (x0, y0) = polar_point(cx, cy, a0, r);
        let large_arc = if sweep > PI { 1 } else { 0 };
        let (x1, y1) = polar_point(cx, cy, a1, r);
        push_b(&mut buf, b"<path fill=\"none\" d=\"M");
        push_f2(&mut buf, x0);
        push_b(&mut buf, b",");
        push_f2(&mut buf, y0);
        push_b(&mut buf, b" A");
        push_f2(&mut buf, r);
        push_b(&mut buf, b",");
        push_f2(&mut buf, r);
        push_b(&mut buf, b" 0 ");
        push_i(&mut buf, large_arc);
        push_b(&mut buf, b",1 ");
        push_f2(&mut buf, x1);
        push_b(&mut buf, b",");
        push_f2(&mut buf, y1);
        push_b(&mut buf, b"\"/>");
    }
    push_b(&mut buf, b"</g>");

    push_b(&mut buf, b"<g stroke=\"#eef1f6\" stroke-width=\"1\">");
    let n_spokes = 48usize;
    for k in 0..=n_spokes {
        let a = start_angle + sweep * k as f64 / n_spokes as f64;
        let (x0, y0) = polar_point(cx, cy, a, inner_r);
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

    let mut slots: Vec<HoverSlot> = Vec::with_capacity(n);
    for i in 0..n {
        let ri = row_of(&cfg.categories[i]);
        let a = angle_of(cfg.x_values[i]);
        let r = row_r(ri);
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

    let list_top = 40.0;
    let list_bottom = h as f64 - 20.0;
    let row_step = if n_rows > 1 { (list_bottom - list_top) / (n_rows - 1) as f64 } else { 0.0 };
    for ri in 0..n_rows {
        let ly = list_top + ri as f64 * row_step;
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
    fn perf_rendering_a_large_radial_row_chart_stays_fast() {
        let (x, sizes, cats, colv) = synth(1200, 60);
        let start = std::time::Instant::now();
        let html = render(&cfg(&x, &sizes, &cats, &colv));
        let elapsed = start.elapsed();
        assert!(elapsed.as_millis() < 200, "rendering took too long: {elapsed:?}");
        assert!(!html.is_empty());
    }
}
