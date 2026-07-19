crate::plot_family! {
    pub enum WordCloudVariant default Basic family "wordcloud" {
        Basic    => "basic" | "default" | "spiral" | "rect" | "shape" | "shaped",
        Image    => "image" | "img" | "mask" | "picture" | "photo" | "silhouette",
        LabelMap => "labelmap" | "label_map" | "datamap" | "datamapplot" | "topic_map" | "scatter_labels",
        Network  => "network" | "graph" | "keywords" | "co_occurrence" | "cooccurrence" | "knowledge_graph",
        Bubble   => "bubble" | "bubbles" | "packed" | "circles" | "packing" | "pack",
        Context  => "context" | "semantic" | "infranodus" | "text_network" | "textnetwork" | "force" | "force_directed",
        Neuron   => "neuron" | "neural" | "brain" | "synapse" | "network_glow" | "nodes",
        Cosmos   => "cosmos" | "stars" | "galaxy" | "nebula" | "constellation" | "space" | "phyllotaxis",
    }
}
