
import { LineChart, Line, BarChart, Bar, PieChart, Pie, Cell, XAxis, YAxis, CartesianGrid, Tooltip, Legend, ResponsiveContainer } from 'recharts'
import './Chart.css'

export function EarningsChart({ data }) {
  return (
    <ResponsiveContainer width="100%" height={300}>
      <LineChart data={data}>
        <CartesianGrid strokeDasharray="3 3" stroke="var(--border-color)" />
        <XAxis dataKey="month" stroke="var(--text-secondary)" />
        <YAxis stroke="var(--text-secondary)" />
        <Tooltip 
          contentStyle={{ 
            backgroundColor: 'var(--bg-primary)', 
            border: '1px solid var(--border-color)',
            borderRadius: '0.5rem'
          }}
        />
        <Legend />
        <Line type="monotone" dataKey="earnings" stroke="var(--primary-color)" strokeWidth={2} />
      </LineChart>
    </ResponsiveContainer>
  )
}

export function ProjectDistributionChart({ data }) {
  const COLORS = ['#4F46E5', '#10B981', '#F59E0B', '#EF4444', '#8B5CF6']
  
  return (
    <ResponsiveContainer width="100%" height={300}>
      <PieChart>
        <Pie
          data={data}
          cx="50%"
          cy="50%"
          labelLine={false}
          label={({ name, percent }) => `${name} ${(percent * 100).toFixed(0)}%`}
          outerRadius={100}
          fill="#8884d8"
          dataKey="value"
        >
          {data.map((entry, index) => (
            <Cell key={`cell-${index}`} fill={COLORS[index % COLORS.length]} />
          ))}
        </Pie>
        <Tooltip 
          contentStyle={{ 
            backgroundColor: 'var(--bg-primary)', 
            border: '1px solid var(--border-color)',
            borderRadius: '0.5rem'
          }}
        />
      </PieChart>
    </ResponsiveContainer>
  )
}

export function PerformanceChart({ data }) {
  return (
    <ResponsiveContainer width="100%" height={300}>
      <BarChart data={data}>
        <CartesianGrid strokeDasharray="3 3" stroke="var(--border-color)" />
        <XAxis dataKey="category" stroke="var(--text-secondary)" />
        <YAxis stroke="var(--text-secondary)" />
        <Tooltip 
          contentStyle={{ 
            backgroundColor: 'var(--bg-primary)', 
            border: '1px solid var(--border-color)',
            borderRadius: '0.5rem'
          }}
        />
        <Legend />
        <Bar dataKey="value" fill="var(--secondary-color)" />
      </BarChart>
    </ResponsiveContainer>
  )
}
