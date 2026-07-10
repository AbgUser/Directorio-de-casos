<script>
  let {
    auth,
    isFirstTime = false
  } = $props();

  let password = $state('');
  let confirmPassword = $state('');
  let showPassword = $state(false);
  let showConfirmPassword = $state(false);
  let error = $state('');
  let isSubmitting = $state(false);
  let showResetConfirm = $state(false);

  async function handleSubmit(e) {
    e.preventDefault();
    error = '';

    if (isFirstTime) {
      if (!password || !confirmPassword) {
        error = 'Ambos campos son requeridos.';
        return;
      }
      if (password.length < 4) {
        error = 'La contraseña debe tener al menos 4 caracteres.';
        return;
      }
      if (password !== confirmPassword) {
        error = 'Las contraseñas no coinciden.';
        return;
      }
    } else {
      if (!password) {
        error = 'Ingrese su contraseña.';
        return;
      }
    }

    isSubmitting = true;
    try {
      let result;
      if (isFirstTime) {
        result = await auth.setupPassword(password);
      } else {
        result = await auth.login(password);
      }
      if (!result.success) {
        error = result.error || 'Error de autenticación.';
      }
    } catch (err) {
      error = 'Error de autenticación. Intente de nuevo.';
    } finally {
      isSubmitting = false;
    }
  }
  async function handleReset() {
    isSubmitting = true;
    try {
      const res = await auth.resetApp();
      if (!res.success) error = res.error;
      else showResetConfirm = false;
    } finally {
      isSubmitting = false;
    }
  }


</script>

<div class="login-screen">
  <div class="login-card fade-in">
    <div class="login-header">
      <div class="login-icon" style="background: transparent;">
        <img src="/logo.png" alt="Logo" style="width: 80px; height: 80px; object-fit: contain; filter: drop-shadow(0 4px 8px rgba(0,0,0,0.2));" />
      </div>
      <h1 class="login-title">Directorio General de Casos</h1>
      <p class="login-subtitle">Sistema de Gestión Jurídica</p>
    </div>

    <form class="login-form" onsubmit={handleSubmit}>
      {#if isFirstTime}
        <p class="setup-message">Configure su contraseña de acceso para proteger la información del despacho.</p>
      {/if}

      <div class="form-field">
        <label class="field-label" for="password">
          {isFirstTime ? 'Crear contraseña' : 'Contraseña'}
        </label>
        <div class="password-wrapper">
          <input
            id="password"
            type={showPassword ? 'text' : 'password'}
            class="field-input"
            placeholder={isFirstTime ? 'Ingrese una contraseña segura' : 'Ingrese su contraseña'}
            bind:value={password}
            autocomplete="off"
          />
          <button
            type="button"
            class="toggle-password"
            onclick={() => showPassword = !showPassword}
            aria-label={showPassword ? 'Ocultar contraseña' : 'Mostrar contraseña'}
          >
            {#if showPassword}
              <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <path d="M17.94 17.94A10.07 10.07 0 0 1 12 20c-7 0-11-8-11-8a18.45 18.45 0 0 1 5.06-5.94M9.9 4.24A9.12 9.12 0 0 1 12 4c7 0 11 8 11 8a18.5 18.5 0 0 1-2.16 3.19m-6.72-1.07a3 3 0 1 1-4.24-4.24"/>
                <line x1="1" y1="1" x2="23" y2="23"/>
              </svg>
            {:else}
              <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <path d="M1 12s4-8 11-8 11 8 11 8-4 8-11 8-11-8-11-8z"/>
                <circle cx="12" cy="12" r="3"/>
              </svg>
            {/if}
          </button>
        </div>
      </div>

      {#if isFirstTime}
        <div class="form-field">
          <label class="field-label" for="confirm-password">Confirmar contraseña</label>
          <div class="password-wrapper">
            <input
              id="confirm-password"
              type={showConfirmPassword ? 'text' : 'password'}
              class="field-input"
              placeholder="Repita la contraseña"
              bind:value={confirmPassword}
              autocomplete="new-password"
            />
            <button
              type="button"
              class="toggle-password"
              onclick={() => showConfirmPassword = !showConfirmPassword}
              aria-label={showConfirmPassword ? 'Ocultar contraseña' : 'Mostrar contraseña'}
            >
              {#if showConfirmPassword}
                <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                  <path d="M17.94 17.94A10.07 10.07 0 0 1 12 20c-7 0-11-8-11-8a18.45 18.45 0 0 1 5.06-5.94M9.9 4.24A9.12 9.12 0 0 1 12 4c7 0 11 8 11 8a18.5 18.5 0 0 1-2.16 3.19m-6.72-1.07a3 3 0 1 1-4.24-4.24"/>
                  <line x1="1" y1="1" x2="23" y2="23"/>
                </svg>
              {:else}
                <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                  <path d="M1 12s4-8 11-8 11 8 11 8-4 8-11 8-11-8-11-8z"/>
                  <circle cx="12" cy="12" r="3"/>
                </svg>
              {/if}
            </button>
          </div>
        </div>
      {/if}

      {#if error}
        <div class="error-message">
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <circle cx="12" cy="12" r="10"/>
            <line x1="15" y1="9" x2="9" y2="15"/>
            <line x1="9" y1="9" x2="15" y2="15"/>
          </svg>
          {error}
        </div>
      {/if}



      <button
        type="submit"
        class="submit-btn"
        disabled={isSubmitting}
      >
        {#if isSubmitting}
          <span class="spinner"></span>
          Procesando...
        {:else}
          {isFirstTime ? 'Configurar acceso' : 'Ingresar'}
        {/if}
      </button>

      {#if !isFirstTime}
        <div class="forgot-password">
          <button type="button" class="btn-link" onclick={() => showResetConfirm = true}>
            ¿Olvidaste tu contraseña?
          </button>
        </div>
      {/if}
    </form>

    {#if showResetConfirm}
      <div class="reset-overlay">
        <div class="reset-dialog fade-in">
          <h3>Restablecer Aplicación</h3>
          <p>
            Si olvidaste tu contraseña, la única forma de volver a ingresar es restableciendo la aplicación, lo que <strong>eliminará permanentemente</strong> todos los datos, clientes y casos registrados actualmente.
          </p>
          <div class="reset-actions">
            <button class="btn-ghost" onclick={() => showResetConfirm = false}>Cancelar</button>
            <button class="btn-danger" onclick={handleReset} disabled={isSubmitting}>Sí, eliminar todo</button>
          </div>
        </div>
      </div>
    {/if}
  </div>
</div>

<style>
  .login-screen {
    position: fixed;
    inset: 0;
    display: flex;
    align-items: center;
    justify-content: center;
    background: var(--bg-primary);
    z-index: 9999;
  }

  .login-card {
    width: 100%;
    max-width: 400px;
    background: var(--bg-secondary);
    border: 1px solid var(--border-default);
    border-radius: var(--radius-xl);
    box-shadow: var(--shadow-lg);
    padding: var(--sp-10);
  }

  .login-header {
    text-align: center;
    margin-bottom: var(--sp-8);
  }

  .login-icon {
    display: flex;
    justify-content: center;
    margin-bottom: var(--sp-5);
  }

  .login-title {
    font-size: 1.375rem;
    font-weight: 700;
    color: var(--text-primary);
    letter-spacing: -0.02em;
    margin-bottom: var(--sp-1);
  }

  .login-subtitle {
    font-size: 0.875rem;
    color: var(--text-secondary);
  }

  .login-form {
    display: flex;
    flex-direction: column;
    gap: var(--sp-5);
  }

  .setup-message {
    font-size: 0.8125rem;
    color: var(--text-secondary);
    text-align: center;
    line-height: 1.5;
  }

  .form-field {
    display: flex;
    flex-direction: column;
    gap: var(--sp-1);
  }

  .field-label {
    font-size: 0.8125rem;
    font-weight: 500;
    color: var(--text-secondary);
  }

  .password-wrapper {
    position: relative;
    display: flex;
    align-items: center;
  }

  .field-input {
    width: 100%;
    padding: var(--sp-3) var(--sp-10) var(--sp-3) var(--sp-3);
    background: var(--bg-input);
    border: 1px solid var(--border-default);
    border-radius: var(--radius-md);
    color: var(--text-primary);
    font-family: var(--font-sans);
    font-size: 0.875rem;
    line-height: 1.5;
    transition: all var(--ease-fast);
    outline: none;
  }

  .field-input::placeholder {
    color: var(--text-muted);
  }

  .field-input:focus {
    border-color: var(--accent-blue);
    box-shadow: var(--shadow-glow-blue);
  }

  .toggle-password {
    position: absolute;
    right: var(--sp-2);
    display: flex;
    align-items: center;
    justify-content: center;
    width: 32px;
    height: 32px;
    background: transparent;
    border: none;
    border-radius: var(--radius-sm);
    color: var(--text-muted);
    cursor: pointer;
    transition: color var(--ease-fast);
  }

  .toggle-password:hover {
    color: var(--text-primary);
  }

  .error-message {
    display: flex;
    align-items: center;
    gap: var(--sp-2);
    font-size: 0.8125rem;
    color: var(--accent-red);
    padding: var(--sp-2) var(--sp-3);
    background: var(--accent-red-soft);
    border-radius: var(--radius-md);
  }

  .submit-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: var(--sp-2);
    width: 100%;
    padding: var(--sp-3) var(--sp-4);
    background: var(--accent-blue);
    color: #fff;
    border: 1px solid var(--accent-blue);
    border-radius: var(--radius-md);
    font-family: var(--font-sans);
    font-size: 0.9375rem;
    font-weight: 600;
    cursor: pointer;
    transition: all var(--ease-fast);
    margin-top: var(--sp-2);
  }

  .submit-btn:hover:not(:disabled) {
    background: var(--accent-blue-hover);
    border-color: var(--accent-blue-hover);
  }

  .submit-btn:active:not(:disabled) {
    transform: scale(0.98);
  }

  .submit-btn:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }

  .spinner {
    width: 16px;
    height: 16px;
    border: 2px solid rgba(255, 255, 255, 0.3);
    border-top-color: #fff;
    border-radius: 50%;
    animation: spin 0.6s linear infinite;
  }

  .forgot-password {
    text-align: center;
    margin-top: var(--sp-2);
  }

  .btn-link {
    background: none; border: none; color: var(--text-muted); font-size: 0.8125rem;
    cursor: pointer; text-decoration: underline; transition: color var(--ease-fast);
  }
  .btn-link:hover { color: var(--text-primary); }

  .reset-overlay {
    position: absolute; inset: 0; background: rgba(0,0,0,0.8); backdrop-filter: blur(4px);
    display: flex; align-items: center; justify-content: center; z-index: 10; border-radius: var(--radius-xl);
  }
  .reset-dialog {
    background: var(--bg-primary); padding: var(--sp-6); border-radius: var(--radius-md);
    border: 1px solid var(--border-focus); max-width: 85%;
  }
  .reset-dialog h3 { color: var(--accent-red); margin-bottom: var(--sp-2); font-size: 1.125rem; }
  .reset-dialog p { font-size: 0.875rem; color: var(--text-secondary); line-height: 1.5; margin-bottom: var(--sp-4); }
  .reset-dialog strong { color: var(--text-primary); }
  
  .reset-actions { display: flex; justify-content: flex-end; gap: var(--sp-3); }
  .btn-ghost { background: none; border: none; color: var(--text-muted); cursor: pointer; padding: var(--sp-2) var(--sp-3); }
  .btn-danger { background: var(--accent-red-soft); color: var(--accent-red); border: none; padding: var(--sp-2) var(--sp-3); border-radius: var(--radius-sm); cursor: pointer; }

  .biometric-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 100%;
    padding: var(--sp-3) var(--sp-4);
    background: var(--bg-primary);
    color: var(--text-primary);
    border: 1px solid var(--border-focus);
    border-radius: var(--radius-md);
    font-family: var(--font-sans);
    font-size: 0.9375rem;
    font-weight: 500;
    cursor: pointer;
    transition: all var(--ease-fast);
  }

  .biometric-btn:hover:not(:disabled) {
    background: var(--bg-hover);
    border-color: var(--text-secondary);
  }

  .biometric-btn:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }

  .divider {
    display: flex;
    align-items: center;
    text-align: center;
    color: var(--text-muted);
    font-size: 0.75rem;
    margin: var(--sp-1) 0;
  }
  .divider::before, .divider::after {
    content: '';
    flex: 1;
    border-bottom: 1px solid var(--border-default);
  }
  .divider span {
    padding: 0 var(--sp-3);
  }
</style>
