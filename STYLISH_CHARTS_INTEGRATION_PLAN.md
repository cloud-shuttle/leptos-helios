# 🎨 Stylish Charts Integration: leptos-helios + leptos-shadcn-ui

## 🎯 **Current State Analysis**

### **leptos-helios Current Capabilities**
- ✅ **Basic styling system** with Tailwind CSS classes
- ✅ **Theme support** (dark, light, neon themes)
- ✅ **Chart types** (Line, Bar, Scatter, Area, etc.)
- ✅ **WebGPU rendering** with fallbacks
- ✅ **Type-safe API** with compile-time validation

### **leptos-shadcn-ui Available Components**
- ✅ **25+ components** (Button, Card, Input, Alert, etc.)
- ✅ **Tailwind CSS integration** with design tokens
- ✅ **Accessibility features** (ARIA, keyboard navigation)
- ✅ **Dark/light mode** support
- ✅ **Responsive design** utilities

### **Current Gap**
- ❌ **No visual demos** showing stylish charts
- ❌ **Limited styling integration** between chart engine and UI components
- ❌ **No modern dashboard examples** with shadcn-ui components

---

## 🚀 **Integration Strategy**

### **Phase 1: Enhanced Chart Components**

Create styled chart components that combine leptos-helios rendering with shadcn-ui styling:

```rust
//! Enhanced chart components with shadcn-ui styling

use leptos::*;
use leptos_shadcn_ui::*;
use leptos_helios::chart::*;

/// Styled chart container with shadcn-ui Card
#[component]
pub fn StyledChartContainer(
    #[prop(into)] title: String,
    #[prop(into)] description: Option<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <Card class="w-full">
            <CardHeader>
                <CardTitle>{title}</CardTitle>
                {if let Some(desc) = description {
                    view! { <CardDescription>{desc}</CardDescription> }
                } else {
                    view! {}
                }}
            </CardHeader>
            <CardContent class="p-0">
                <div class="aspect-video w-full">
                    {children()}
                </div>
            </CardContent>
        </Card>
    }
}

/// Styled line chart with modern design
#[component]
pub fn StyledLineChart(
    #[prop(into)] data: Vec<DataPoint>,
    #[prop(into)] title: String,
    #[prop(optional)] show_tooltip: bool,
    #[prop(optional)] show_legend: bool,
) -> impl IntoView {
    let chart_spec = create_styled_line_chart(data, title.clone());
    
    view! {
        <StyledChartContainer title=title description=Some("Interactive line chart with modern styling".to_string())>
            <div class="relative h-full w-full">
                <HeliosChart spec=chart_spec />
                {if show_tooltip {
                    view! {
                        <div class="absolute top-2 right-2">
                            <Badge variant="secondary">
                                <Info class="mr-1 h-3 w-3" />
                                "Hover for details"
                            </Badge>
                        </div>
                    }
                } else {
                    view! {}
                }}
            </div>
        </StyledChartContainer>
    }
}
```

### **Phase 2: Modern Dashboard Layout**

Create a complete dashboard with shadcn-ui components:

```rust
/// Modern analytics dashboard
#[component]
pub fn AnalyticsDashboard() -> impl IntoView {
    let (selected_metric, set_selected_metric) = create_signal("revenue".to_string());
    let (date_range, set_date_range) = create_signal("30d".to_string());
    
    view! {
        <div class="min-h-screen bg-background">
            <div class="container mx-auto p-6 space-y-6">
                // Header with controls
                <div class="flex items-center justify-between">
                    <div>
                        <h1 class="text-3xl font-bold tracking-tight">"Analytics Dashboard"</h1>
                        <p class="text-muted-foreground">"Real-time insights and metrics"</p>
                    </div>
                    <div class="flex items-center space-x-2">
                        <Select value=selected_metric on_change=move |v| set_selected_metric(v)>
                            <SelectTrigger class="w-[180px]">
                                <SelectValue placeholder="Select metric" />
                            </SelectTrigger>
                            <SelectContent>
                                <SelectItem value="revenue">"Revenue"</SelectItem>
                                <SelectItem value="users">"Users"</SelectItem>
                                <SelectItem value="conversion">"Conversion"</SelectItem>
                            </SelectContent>
                        </Select>
                        <Select value=date_range on_change=move |v| set_date_range(v)>
                            <SelectTrigger class="w-[120px]">
                                <SelectValue placeholder="Period" />
                            </SelectTrigger>
                            <SelectContent>
                                <SelectItem value="7d">"7 days"</SelectItem>
                                <SelectItem value="30d">"30 days"</SelectItem>
                                <SelectItem value="90d">"90 days"</SelectItem>
                            </SelectContent>
                        </Select>
                    </div>
                </div>

                // Metrics cards
                <div class="grid gap-4 md:grid-cols-2 lg:grid-cols-4">
                    <MetricCard 
                        title="Total Revenue" 
                        value="$45,231.89" 
                        change="+20.1%" 
                        trend="up" 
                    />
                    <MetricCard 
                        title="Active Users" 
                        value="2,350" 
                        change="+180.1%" 
                        trend="up" 
                    />
                    <MetricCard 
                        title="Conversion Rate" 
                        value="12.5%" 
                        change="-19%" 
                        trend="down" 
                    />
                    <MetricCard 
                        title="Avg. Session" 
                        value="4m 32s" 
                        change="+2%" 
                        trend="up" 
                    />
                </div>

                // Charts grid
                <div class="grid gap-6 md:grid-cols-2">
                    <StyledLineChart 
                        data=generate_revenue_data() 
                        title="Revenue Trend" 
                        show_tooltip=true 
                        show_legend=true 
                    />
                    <StyledBarChart 
                        data=generate_user_data() 
                        title="User Growth" 
                        show_tooltip=true 
                    />
                </div>

                <div class="grid gap-6 md:grid-cols-3">
                    <StyledAreaChart 
                        data=generate_conversion_data() 
                        title="Conversion Funnel" 
                    />
                    <StyledScatterPlot 
                        data=generate_correlation_data() 
                        title="User Behavior" 
                    />
                    <StyledPieChart 
                        data=generate_traffic_data() 
                        title="Traffic Sources" 
                    />
                </div>
            </div>
        </div>
    }
}
```

### **Phase 3: Interactive Chart Controls**

Add interactive controls using shadcn-ui components:

```rust
/// Interactive chart with controls
#[component]
pub fn InteractiveChart(
    #[prop(into)] initial_data: Vec<DataPoint>,
    #[prop(into)] title: String,
) -> impl IntoView {
    let (data, set_data) = create_signal(initial_data);
    let (chart_type, set_chart_type) = create_signal("line".to_string());
    let (show_grid, set_show_grid) = create_signal(true);
    let (animation_enabled, set_animation_enabled) = create_signal(true);
    
    view! {
        <Card class="w-full">
            <CardHeader>
                <div class="flex items-center justify-between">
                    <div>
                        <CardTitle>{title}</CardTitle>
                        <CardDescription>"Interactive chart with real-time controls"</CardDescription>
                    </div>
                    <div class="flex items-center space-x-2">
                        <Button variant="outline" size="sm">
                            <Download class="mr-2 h-4 w-4" />
                            "Export"
                        </Button>
                        <Button variant="outline" size="sm">
                            <Settings class="mr-2 h-4 w-4" />
                            "Settings"
                        </Button>
                    </div>
                </div>
            </CardHeader>
            <CardContent class="space-y-4">
                // Controls panel
                <div class="flex flex-wrap items-center gap-4 p-4 bg-muted/50 rounded-lg">
                    <div class="flex items-center space-x-2">
                        <Label for="chart-type">"Chart Type"</Label>
                        <Select value=chart_type on_change=move |v| set_chart_type(v)>
                            <SelectTrigger class="w-[120px]">
                                <SelectValue />
                            </SelectTrigger>
                            <SelectContent>
                                <SelectItem value="line">"Line"</SelectItem>
                                <SelectItem value="bar">"Bar"</SelectItem>
                                <SelectItem value="area">"Area"</SelectItem>
                                <SelectItem value="scatter">"Scatter"</SelectItem>
                            </SelectContent>
                        </Select>
                    </div>
                    
                    <div class="flex items-center space-x-2">
                        <Switch 
                            id="show-grid" 
                            checked=show_grid 
                            on_change=move |v| set_show_grid(v) 
                        />
                        <Label for="show-grid">"Show Grid"</Label>
                    </div>
                    
                    <div class="flex items-center space-x-2">
                        <Switch 
                            id="animation" 
                            checked=animation_enabled 
                            on_change=move |v| set_animation_enabled(v) 
                        />
                        <Label for="animation">"Animation"</Label>
                    </div>
                    
                    <Button 
                        variant="outline" 
                        size="sm"
                        on:click=move |_| {
                            // Generate new random data
                            set_data(generate_random_data());
                        }
                    >
                        <RefreshCw class="mr-2 h-4 w-4" />
                        "Refresh Data"
                    </Button>
                </div>

                // Chart area
                <div class="aspect-video w-full">
                    <HeliosChart 
                        spec=create_dynamic_chart(
                            data.get(), 
                            chart_type.get(), 
                            show_grid.get(), 
                            animation_enabled.get()
                        ) 
                    />
                </div>
            </CardContent>
        </Card>
    }
}
```

---

## 🎨 **Styling Enhancements**

### **1. Color Palette Integration**

```rust
/// shadcn-ui color palette for charts
pub struct ShadcnColorPalette {
    pub primary: String,      // hsl(var(--primary))
    pub secondary: String,    // hsl(var(--secondary))
    pub accent: String,       // hsl(var(--accent))
    pub muted: String,        // hsl(var(--muted))
    pub destructive: String,  // hsl(var(--destructive))
    pub success: String,      // hsl(var(--success))
    pub warning: String,      // hsl(var(--warning))
    pub info: String,         // hsl(var(--info))
}

impl Default for ShadcnColorPalette {
    fn default() -> Self {
        Self {
            primary: "hsl(var(--primary))".to_string(),
            secondary: "hsl(var(--secondary))".to_string(),
            accent: "hsl(var(--accent))".to_string(),
            muted: "hsl(var(--muted))".to_string(),
            destructive: "hsl(var(--destructive))".to_string(),
            success: "hsl(var(--success))".to_string(),
            warning: "hsl(var(--warning))".to_string(),
            info: "hsl(var(--info))".to_string(),
        }
    }
}
```

### **2. Responsive Design**

```rust
/// Responsive chart container
#[component]
pub fn ResponsiveChart(
    #[prop(into)] spec: ChartSpec,
    #[prop(optional)] class: Option<String>,
) -> impl IntoView {
    view! {
        <div class=format!(
            "w-full h-64 sm:h-80 md:h-96 lg:h-[500px] {}",
            class.unwrap_or_default()
        )>
            <HeliosChart spec=spec />
        </div>
    }
}
```

### **3. Loading States**

```rust
/// Chart with loading state
#[component]
pub fn ChartWithLoading(
    #[prop(into)] spec: Option<ChartSpec>,
    #[prop(optional)] loading: bool,
) -> impl IntoView {
    view! {
        <div class="relative w-full h-64">
            {if loading || spec.is_none() {
                view! {
                    <div class="absolute inset-0 flex items-center justify-center">
                        <div class="flex flex-col items-center space-y-2">
                            <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-primary"></div>
                            <p class="text-sm text-muted-foreground">"Loading chart..."</p>
                        </div>
                    </div>
                }
            } else {
                view! {
                    <HeliosChart spec=spec.unwrap() />
                }
            }}
        </div>
    }
}
```

---

## 📊 **Demo Examples**

### **1. Sales Analytics Dashboard**

```rust
/// Complete sales analytics dashboard
#[component]
pub fn SalesAnalyticsDashboard() -> impl IntoView {
    view! {
        <div class="min-h-screen bg-background">
            <div class="container mx-auto p-6 space-y-6">
                // Header
                <div class="flex items-center justify-between">
                    <div>
                        <h1 class="text-3xl font-bold">"Sales Analytics"</h1>
                        <p class="text-muted-foreground">"Track your sales performance"</p>
                    </div>
                    <div class="flex items-center space-x-2">
                        <Button variant="outline">
                            <Calendar class="mr-2 h-4 w-4" />
                            "Last 30 days"
                        </Button>
                        <Button>
                            <Download class="mr-2 h-4 w-4" />
                            "Export Report"
                        </Button>
                    </div>
                </div>

                // KPI Cards
                <div class="grid gap-4 md:grid-cols-2 lg:grid-cols-4">
                    <MetricCard 
                        title="Total Sales" 
                        value="$125,430" 
                        change="+12.5%" 
                        trend="up" 
                        icon=TrendingUp 
                    />
                    <MetricCard 
                        title="Orders" 
                        value="1,234" 
                        change="+8.2%" 
                        trend="up" 
                        icon=ShoppingCart 
                    />
                    <MetricCard 
                        title="Avg. Order Value" 
                        value="$101.60" 
                        change="-2.1%" 
                        trend="down" 
                        icon=DollarSign 
                    />
                    <MetricCard 
                        title="Conversion Rate" 
                        value="3.2%" 
                        change="+0.5%" 
                        trend="up" 
                        icon=Target 
                    />
                </div>

                // Main charts
                <div class="grid gap-6 md:grid-cols-2">
                    <StyledLineChart 
                        data=generate_sales_trend_data() 
                        title="Sales Trend" 
                        show_tooltip=true 
                        show_legend=true 
                    />
                    <StyledBarChart 
                        data=generate_category_data() 
                        title="Sales by Category" 
                        show_tooltip=true 
                    />
                </div>

                // Secondary charts
                <div class="grid gap-6 md:grid-cols-3">
                    <StyledAreaChart 
                        data=generate_revenue_data() 
                        title="Revenue Streams" 
                    />
                    <StyledScatterPlot 
                        data=generate_correlation_data() 
                        title="Price vs Volume" 
                    />
                    <StyledPieChart 
                        data=generate_region_data() 
                        title="Sales by Region" 
                    />
                </div>
            </div>
        </div>
    }
}
```

### **2. Real-time Monitoring Dashboard**

```rust
/// Real-time monitoring with WebSocket updates
#[component]
pub fn RealTimeMonitoringDashboard() -> impl IntoView {
    let (metrics, set_metrics) = create_signal(initial_metrics());
    let (is_connected, set_is_connected) = create_signal(false);
    
    // WebSocket connection for real-time updates
    create_effect(move |_| {
        // WebSocket connection logic
        spawn_local(async move {
            // Connect to WebSocket and update metrics
        });
    });
    
    view! {
        <div class="min-h-screen bg-background">
            <div class="container mx-auto p-6 space-y-6">
                // Header with connection status
                <div class="flex items-center justify-between">
                    <div>
                        <h1 class="text-3xl font-bold">"System Monitoring"</h1>
                        <p class="text-muted-foreground">"Real-time system metrics"</p>
                    </div>
                    <div class="flex items-center space-x-2">
                        <Badge variant=if is_connected.get() { "default" } else { "destructive" }>
                            {if is_connected.get() {
                                view! { <Wifi class="mr-1 h-3 w-3" /> "Connected" }
                            } else {
                                view! { <WifiOff class="mr-1 h-3 w-3" /> "Disconnected" }
                            }}
                        </Badge>
                        <Button variant="outline" size="sm">
                            <Settings class="mr-2 h-4 w-4" />
                            "Settings"
                        </Button>
                    </div>
                </div>

                // System metrics
                <div class="grid gap-4 md:grid-cols-2 lg:grid-cols-4">
                    <MetricCard 
                        title="CPU Usage" 
                        value=format!("{:.1}%", metrics.get().cpu_usage) 
                        change=format!("{:.1}%", metrics.get().cpu_change) 
                        trend=if metrics.get().cpu_change > 0.0 { "up" } else { "down" } 
                        icon=Cpu 
                    />
                    <MetricCard 
                        title="Memory Usage" 
                        value=format!("{:.1}%", metrics.get().memory_usage) 
                        change=format!("{:.1}%", metrics.get().memory_change) 
                        trend=if metrics.get().memory_change > 0.0 { "up" } else { "down" } 
                        icon=MemoryStick 
                    />
                    <MetricCard 
                        title="Network I/O" 
                        value=format!("{:.1} MB/s", metrics.get().network_io) 
                        change=format!("{:.1}%", metrics.get().network_change) 
                        trend=if metrics.get().network_change > 0.0 { "up" } else { "down" } 
                        icon=Network 
                    />
                    <MetricCard 
                        title="Active Users" 
                        value=metrics.get().active_users.to_string() 
                        change=format!("{:.1}%", metrics.get().users_change) 
                        trend=if metrics.get().users_change > 0.0 { "up" } else { "down" } 
                        icon=Users 
                    />
                </div>

                // Real-time charts
                <div class="grid gap-6 md:grid-cols-2">
                    <RealtimeLineChart 
                        data=metrics.get().cpu_history 
                        title="CPU Usage Over Time" 
                        max_points=100 
                    />
                    <RealtimeLineChart 
                        data=metrics.get().memory_history 
                        title="Memory Usage Over Time" 
                        max_points=100 
                    />
                </div>
            </div>
        </div>
    }
}
```

---

## 🚀 **Implementation Plan**

### **Phase 1: Foundation (Week 1-2)**
1. ✅ **Setup leptos-shadcn-ui integration**
2. ✅ **Create base styled chart components**
3. ✅ **Implement color palette integration**
4. ✅ **Add responsive design utilities**

### **Phase 2: Components (Week 3-4)**
1. ✅ **Build styled chart containers**
2. ✅ **Create interactive controls**
3. ✅ **Add loading states and error handling**
4. ✅ **Implement metric cards**

### **Phase 3: Dashboards (Week 5-6)**
1. ✅ **Create analytics dashboard**
2. ✅ **Build real-time monitoring dashboard**
3. ✅ **Add export and sharing features**
4. ✅ **Implement theme switching**

### **Phase 4: Polish (Week 7-8)**
1. ✅ **Add animations and transitions**
2. ✅ **Optimize performance**
3. ✅ **Create comprehensive documentation**
4. ✅ **Build demo showcase**

---

## 📦 **Dependencies**

Add to `Cargo.toml`:

```toml
[dependencies]
leptos-shadcn-ui = "0.1"
leptos-helios = "0.8.1"
leptos = "0.8"
```

---

## 🎯 **Expected Results**

After implementing this integration, you'll have:

1. **🎨 Beautiful, modern charts** with shadcn-ui styling
2. **📱 Responsive dashboards** that work on all devices
3. **⚡ Interactive controls** for real-time chart customization
4. **🌙 Dark/light mode** support with seamless theme switching
5. **♿ Accessibility features** built-in with ARIA support
6. **📊 Professional dashboards** ready for production use

This combination will make leptos-helios charts look as polished and modern as the best JavaScript charting libraries, while maintaining the performance and type safety advantages of Rust!
