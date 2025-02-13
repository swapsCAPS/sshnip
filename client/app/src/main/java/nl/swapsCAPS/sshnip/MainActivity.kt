package nl.swapsCAPS.sshnip

import android.content.ClipboardManager
import android.content.Context
import android.os.Bundle
import android.widget.Toast
import androidx.appcompat.app.AppCompatActivity
import androidx.core.widget.doOnTextChanged
import androidx.datastore.core.DataStore
import androidx.datastore.preferences.core.Preferences
import androidx.datastore.preferences.core.edit
import androidx.datastore.preferences.core.stringPreferencesKey
import androidx.datastore.preferences.preferencesDataStore
import androidx.lifecycle.lifecycleScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.flow.first
import kotlinx.coroutines.launch
import kotlinx.coroutines.withContext
import nl.swapsCAPS.sshnip.databinding.ActivityMainBinding
import okhttp3.MediaType.Companion.toMediaTypeOrNull
import okhttp3.OkHttpClient
import okhttp3.Request
import okhttp3.RequestBody.Companion.toRequestBody
import okhttp3.Response
import java.time.Instant

val Context.dataStore: DataStore<Preferences> by preferencesDataStore("setting")
val PREFS_VERSION = "v2"
val URL_KEY = stringPreferencesKey("url_${PREFS_VERSION}")
val API_KEY_KEY = stringPreferencesKey("api_key_${PREFS_VERSION}")
val LOG_KEY = stringPreferencesKey("logs_${PREFS_VERSION}")

class MainActivity : AppCompatActivity() {
  private lateinit var binding: ActivityMainBinding
  private var url: String = ""
  private var apiKey: String = ""

  override fun onResume() {
    super.onResume()

    val clipboard = getSystemService(CLIPBOARD_SERVICE) as ClipboardManager

    binding = ActivityMainBinding.inflate(layoutInflater)
    setContentView(binding.root)

    lifecycleScope.launch {
      val preferences = dataStore.data.first()

      apiKey = preferences[API_KEY_KEY].orEmpty()
      url = preferences[URL_KEY].orEmpty()
      val logLines = preferences[LOG_KEY].orEmpty()

      binding.editTextTexApiKey.setText(apiKey)
      binding.editTextTextURL.setText(url)
      binding.resultView.setText(logLines)

      val clipData = clipboard.primaryClip?.getItemAt(0)?.text?.toString()

      if (url.isNotEmpty() && apiKey.isNotEmpty() && !clipData.isNullOrEmpty()) {
        var text = ""
        try {
          val response = send(url, apiKey, clipData)
          if (response.isSuccessful) {
            Toast.makeText(this@MainActivity, "Sent!", Toast.LENGTH_SHORT).show()
            finish()
          } else {
            text = ("something went wrong ${response.code}, ${response.body}")
          }
        } catch (e: Exception) {
          text = "something went wrong ${e}"
        }

        val time = Instant.now()

        val lines = "${time} - ${text}\r\n" +
          // This _EOL thing is a really ugly! Do better
          logLines.split("_EOL").take(9).joinToString("\r\n")

        binding.resultView.setText(lines)

        dataStore.edit { settings ->
          settings[LOG_KEY] = lines.split("\r\n").joinToString("_EOL")
        }
      }
    }

    binding.editTextTexApiKey.doOnTextChanged { text, start, before, count ->
      // Debounce?
      lifecycleScope.launch {
        dataStore.edit { settings ->
          settings[API_KEY_KEY] = text.toString()
        }
      }
    }

    binding.editTextTextURL.doOnTextChanged { text, start, before, count ->
      lifecycleScope.launch {
        dataStore.edit { settings ->
          settings[URL_KEY] = text.toString()
        }
      }
    }
  }

  private suspend fun send(url: String, apiKey: String, payload: String): Response =
    withContext(Dispatchers.IO) {
      val client = OkHttpClient()

      val requestBody = payload.toRequestBody("text/plain".toMediaTypeOrNull())

      // Build the request with headers
      val request = Request.Builder()
        .url(url)
        .post(requestBody)
        .addHeader("x-api-key", apiKey)
        .build()

      // Execute the request
      return@withContext client.newCall(request).execute()
    }
}
