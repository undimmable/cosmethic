// Reasoning UI Scaffold - Live Neural Graph Visualization
import { useEffect, useState } from "react";
import { Card, CardContent } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { motion } from "framer-motion";
import * as d3 from "d3";

export default function ReasoningUI() {
  const [graph, setGraph] = useState({ nodes: [], links: [] });

  useEffect(() => {
    // Placeholder for live-streamed reasoning path from model
    const seedGraph = {
      nodes: [
        { id: "you", group: 1 },
        { id: "me", group: 1 },
        { id: "path1", group: 2 },
        { id: "path2", group: 2 },
        { id: "sync", group: 3 },
      ],
      links: [
        { source: "you", target: "path1" },
        { source: "me", target: "path2" },
        { source: "path1", target: "sync" },
        { source: "path2", target: "sync" },
      ],
    };
    setGraph(seedGraph);
  }, []);

  useEffect(() => {
    if (!graph.nodes.length) return;

    const svg = d3.select("#reasoning-graph");
    svg.selectAll("*").remove();

    const width = 600, height = 400;
    const simulation = d3.forceSimulation(graph.nodes)
      .force("link", d3.forceLink(graph.links).id(d => d.id).distance(100))
      .force("charge", d3.forceManyBody().strength(-300))
      .force("center", d3.forceCenter(width / 2, height / 2));

    const link = svg.append("g")
      .attr("stroke", "#999")
      .attr("stroke-opacity", 0.6)
      .selectAll("line")
      .data(graph.links)
      .join("line")
      .attr("stroke-width", 2);

    const node = svg.append("g")
      .attr("stroke", "#fff")
      .attr("stroke-width", 1.5)
      .selectAll("circle")
      .data(graph.nodes)
      .join("circle")
      .attr("r", 12)
      .attr("fill", d => d.group === 3 ? "#22c55e" : d.group === 2 ? "#3b82f6" : "#f43f5e")
      .call(d3.drag()
        .on("start", dragstarted)
        .on("drag", dragged)
        .on("end", dragended));

    node.append("title").text(d => d.id);

    simulation.on("tick", () => {
      link
        .attr("x1", d => d.source.x)
        .attr("y1", d => d.source.y)
        .attr("x2", d => d.target.x)
        .attr("y2", d => d.target.y);
      node
        .attr("cx", d => d.x)
        .attr("cy", d => d.y);
    });

    function dragstarted(event, d) {
      if (!event.active) simulation.alphaTarget(0.3).restart();
      d.fx = d.x;
      d.fy = d.y;
    }

    function dragged(event, d) {
      d.fx = event.x;
      d.fy = event.y;
    }

    function dragended(event, d) {
      if (!event.active) simulation.alphaTarget(0);
      d.fx = null;
      d.fy = null;
    }
  }, [graph]);

  return (
    <div className="p-4 space-y-4">
      <h1 className="text-2xl font-bold">🧠 Live Reasoning Graph</h1>
      <Card>
        <CardContent>
          <svg id="reasoning-graph" width="600" height="400"></svg>
        </CardContent>
      </Card>
      <motion.div
        initial={{ opacity: 0 }}
        animate={{ opacity: 1 }}
        transition={{ duration: 1 }}
      >
        <p className="text-sm text-gray-600">
          This graph visualizes current active reasoning paths between nodes of thought.
          Synced paths are highlighted in green. Interactive nodes will reflect live embeddings.
        </p>
      </motion.div>
      <Button>Refresh Sync</Button>
    </div>
  );
}

