<script lang="ts">
let celsius = $state(0);
let fahrenheit = $state(32);

function updateFahrenheit() {
  fahrenheit = Math.round(((celsius * 9) / 5 + 32) * 100) / 100;
}

function updateCelsius() {
  celsius = Math.round((((fahrenheit - 32) * 5) / 9) * 100) / 100;
}
</script>

<div class="max-w-md mx-auto">
	<h1 class="text-3xl font-bold mb-6">Temperature Converter</h1>

	<div class="card bg-base-100 shadow-lg">
		<div class="card-body space-y-4">
			<div class="form-control">
				<label class="label" for="celsius">
					<span class="label-text">Celsius</span>
				</label>
				<div class="join">
					<input
						id="celsius"
						type="number"
						step="0.01"
						bind:value={celsius}
						oninput={updateFahrenheit}
						class="input join-item flex-1"
						placeholder="0"
					/>
					<div>
						<span class="btn join-item">°C</span>
					</div>
				</div>
			</div>

			<div class="divider">⇅</div>

			<div class="form-control">
				<label class="label" for="fahrenheit">
					<span class="label-text">Fahrenheit</span>
				</label>
				<div class="join">
					<input
						id="fahrenheit"
						type="number"
						step="0.01"
						bind:value={fahrenheit}
						oninput={updateCelsius}
						class="input input-bordered join-item flex-1"
						placeholder="32"
					/>
					<div>
						<span class="btn join-item">°F</span>
					</div>
				</div>
			</div>
		</div>
	</div>

	<div class="mt-4 text-center text-sm text-base-content opacity-70">
		<p>Enter a temperature value in either field to convert between Celsius and Fahrenheit.</p>
	</div>
</div>