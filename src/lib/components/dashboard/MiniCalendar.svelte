<script>
  let { alertas = [], onSelectDate } = $props();

  let currentDate = $state(new Date());
  let selectedDateStr = $state(null);

  // Derive current month and year
  let currentMonth = $derived(currentDate.getMonth());
  let currentYear = $derived(currentDate.getFullYear());

  let daysInMonth = $derived(new Date(currentYear, currentMonth + 1, 0).getDate());
  let firstDayOfMonth = $derived(new Date(currentYear, currentMonth, 1).getDay());

  const monthNames = ["Enero", "Febrero", "Marzo", "Abril", "Mayo", "Junio", "Julio", "Agosto", "Septiembre", "Octubre", "Noviembre", "Diciembre"];

  // Helper to map date to string YYYY-MM-DD
  const toDateStr = (year, month, day) => {
    return `${year}-${String(month + 1).padStart(2, '0')}-${String(day).padStart(2, '0')}`;
  };

  function prevMonth() {
    currentDate = new Date(currentYear, currentMonth - 1, 1);
  }

  function nextMonth() {
    currentDate = new Date(currentYear, currentMonth + 1, 1);
  }

  function handleDayClick(day) {
    if (!day) return;
    const str = toDateStr(currentYear, currentMonth, day);
    if (selectedDateStr === str) {
      selectedDateStr = null; // deselect
    } else {
      selectedDateStr = str;
    }
    if (onSelectDate) onSelectDate(selectedDateStr);
  }

  // Check if a specific date string has an alert
  function getAlertasForDate(dateStr) {
    return alertas.filter(a => a.fecha_inicio === dateStr);
  }
</script>

<div class="calendar-container">
  <div class="calendar-header">
    <button onclick={prevMonth} class="nav-btn">
      <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="15 18 9 12 15 6"/></svg>
    </button>
    <strong class="month-label">{monthNames[currentMonth]} {currentYear}</strong>
    <button onclick={nextMonth} class="nav-btn">
      <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="9 18 15 12 9 6"/></svg>
    </button>
  </div>

  <div class="calendar-grid">
    <div class="day-name">Do</div>
    <div class="day-name">Lu</div>
    <div class="day-name">Ma</div>
    <div class="day-name">Mi</div>
    <div class="day-name">Ju</div>
    <div class="day-name">Vi</div>
    <div class="day-name">Sa</div>

    {#each Array(firstDayOfMonth) as _}
      <div class="calendar-day empty"></div>
    {/each}

    {#each Array(daysInMonth) as _, i}
      {@const day = i + 1}
      {@const dateStr = toDateStr(currentYear, currentMonth, day)}
      {@const dayAlerts = getAlertasForDate(dateStr)}
      {@const hasAlerts = dayAlerts.length > 0}
      {@const isSelected = selectedDateStr === dateStr}
      {@const isToday = dateStr === toDateStr(new Date().getFullYear(), new Date().getMonth(), new Date().getDate())}
      
      <div 
        class="calendar-day {hasAlerts ? 'has-event' : ''} {isSelected ? 'selected' : ''} {isToday ? 'today' : ''}"
        onclick={() => handleDayClick(day)}
        role="button"
        tabindex="0"
        onkeydown={(e) => e.key === 'Enter' && handleDayClick(day)}
      >
        <span class="day-number">{day}</span>
        {#if hasAlerts}
          <div class="event-dots">
            {#each dayAlerts.slice(0, 3) as al}
              <div class="dot {al.urgencia || 'default'}"></div>
            {/each}
          </div>
        {/if}
      </div>
    {/each}
  </div>
</div>

<style>
  .calendar-container {
    background: var(--bg-surface);
    border: 1px solid var(--border-default);
    border-radius: var(--radius-lg);
    padding: var(--sp-4);
  }

  .calendar-header {
    display: flex; justify-content: space-between; align-items: center;
    margin-bottom: var(--sp-4);
  }

  .month-label { font-size: 0.9375rem; color: var(--text-primary); font-weight: 600; text-transform: capitalize; }
  
  .nav-btn {
    background: none; border: none; cursor: pointer; color: var(--text-secondary);
    border-radius: var(--radius-sm); padding: 4px; display: flex; align-items: center;
    transition: background var(--ease-fast);
  }
  .nav-btn:hover { background: var(--bg-hover); color: var(--text-primary); }

  .calendar-grid {
    display: grid; grid-template-columns: repeat(7, 1fr); gap: 4px;
  }

  .day-name {
    text-align: center; font-size: 0.75rem; color: var(--text-muted);
    font-weight: 600; padding-bottom: var(--sp-2);
  }

  .calendar-day {
    aspect-ratio: 1; display: flex; flex-direction: column; align-items: center; justify-content: center;
    border-radius: var(--radius-md); font-size: 0.875rem; color: var(--text-primary);
    cursor: pointer; transition: all var(--ease-fast);
    position: relative;
  }
  
  .calendar-day.empty { cursor: default; }

  .calendar-day:not(.empty):hover {
    background: var(--bg-hover);
  }

  .calendar-day.has-event .day-number {
    color: var(--accent-blue);
    font-weight: 600;
  }

  .calendar-day.today .day-number {
    font-weight: 700; color: var(--accent-blue); text-decoration: underline;
  }

  .calendar-day.selected {
    background: var(--accent-blue); color: #fff;
  }
  .calendar-day.selected .day-number { color: #fff; }

  .event-dots {
    display: flex; gap: 2px; margin-top: 2px;
  }

  .dot {
    width: 4px; height: 4px; border-radius: 50%;
  }
  .dot.vencido { background: var(--accent-red); }
  .dot.alta { background: var(--accent-red); }
  .dot.media { background: var(--accent-amber); }
  .dot.default { background: var(--text-muted); }
  .calendar-day.selected .dot { background: #fff; }
</style>
