import React from "react";
import { useQuery } from "@tanstack/react-query";
import { Activity, Cpu, HardDrive, Wifi } from "lucide-react";
import { api } from "../../lib/api";

const MetricCard: React.FC<{
  label: string;
  value: string;
  sub?: string;
  icon: React.ElementType;
  color: string;
}> = ({ label, value, sub, icon: Icon, color }) => (
  <div className="bg-surface-elevated border border-surface-border rounded-xl p-5 hover:shadow-lg transition-shadow">
    <div className="flex items-center justify-between mb-3">
      <span className="text-sm text-zinc-400">{label}</span>
      <div className={`p-2 rounded-lg ${color}`}>
        <Icon className="w-5 h-5 text-white" />
      </div>
    </div>
    <div className="text-2xl font-bold">{value}</div>
    {sub && <div className="text-xs text-zinc-500 mt-1">{sub}</div>}
  </div>
);

const Dashboard: React.FC = () => {
  const { data: metrics } = useQuery({
    queryKey: ["metrics"],
    queryFn: api.getMetrics,
    refetchInterval: 2000,
  });

  return (
    <div className="space-y-6">
      <div>
        <h1 className="text-2xl font-bold">Dashboard</h1>
        <p className="text-zinc-400 text-sm mt-1">
          System overview and live metrics
        </p>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
        <MetricCard
          label="CPU Usage"
          value={`${metrics?.cpu_percent.toFixed(1) ?? "--"}%`}
          icon={Cpu}
          color="bg-brand-600"
        />
        <MetricCard
          label="RAM Usage"
          value={`${metrics?.ram_percent.toFixed(1) ?? "--"}%`}
          sub={metrics ? `${metrics.ram_used_mb} / ${metrics.ram_total_mb} MB` : undefined}
          icon={Activity}
          color="bg-emerald-600"
        />
        <MetricCard
          label="Disk Active"
          value={`${metrics?.disk_active_percent.toFixed(1) ?? "--"}%`}
          icon={HardDrive}
          color="bg-amber-600"
        />
        <MetricCard
          label="Network"
          value={`${metrics?.network_down_mbps.toFixed(1) ?? "--"} Mbps`}
          sub={metrics ? `↑ ${metrics.network_up_mbps.toFixed(1)} Mbps` : undefined}
          icon={Wifi}
          color="bg-purple-600"
        />
      </div>

      <div className="bg-surface-elevated border border-surface-border rounded-xl p-6">
        <h2 className="text-lg font-semibold mb-2">Recommended Actions</h2>
        <p className="text-zinc-400 text-sm">
          No recommended actions at this time. Your system looks good!
        </p>
      </div>
    </div>
  );
};

export default Dashboard;
