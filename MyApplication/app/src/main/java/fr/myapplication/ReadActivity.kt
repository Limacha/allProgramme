package fr.myapplication

import android.content.Intent
import android.content.pm.ActivityInfo
import androidx.appcompat.app.AppCompatActivity
import android.os.Bundle
import android.widget.Button
import android.widget.ImageView
import android.widget.TextView
import androidx.fragment.app.Fragment
import com.google.gson.Gson
import com.google.gson.reflect.TypeToken
import fr.myapplication.fragments.DownloadFragment
import fr.myapplication.fragments.ReadFragement

class ReadActivity : AppCompatActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        requestedOrientation = ActivityInfo.SCREEN_ORIENTATION_PORTRAIT
        val scanname = intent.getSerializableExtra("name") as String
        val chapitre = intent.getSerializableExtra("chapitre") as Int

        /* limite max

        //creez liste stock scan
        val scanList = arrayListOf<ScanModel>()

        // Ouvre le fichier JSON à partir des assets
        val inputStream = this.assets.open("scan.json")

        // Lit le contenu du fichier dans une chaîne
        val json = inputStream.bufferedReader().use { it.readText() }

        // Analyse le contenu JSON en tant qu'objet Kotlin
        val gson = Gson()
        val type = object : TypeToken<List<ScanModel>>() {}.type
        val data: MutableList<ScanModel> = gson.fromJson(json, type)

        // Ajoute chaque élément à la liste
        scanList.addAll(data)
        val currentScan = scanList[]

         */

        setContentView(R.layout.activity_read)
        setupComponents(chapitre)
        setupButton(scanname, chapitre)
        loadFragment(ReadFragement(this, scanname, chapitre))
        setupCloseButton()
    }

    private fun setupComponents(chapitre: Int) {
        val textChap = findViewById<TextView>(R.id.textChap)

        textChap.text = chapitre.toString()
    }

    private fun setupButton(scanname: String, chapitre: Int) {
        val nextButton = findViewById<Button>(R.id.buttonNext)
        val previousButton = findViewById<Button>(R.id.buttonPrevious)

        nextButton.setOnClickListener {
            setupComponents(chapitre + 1)
            setupButton(scanname, chapitre + 1)
            loadFragment(ReadFragement(this, scanname, chapitre + 1))

        }
        previousButton.setOnClickListener {
            if (chapitre >= 1) {
                setupComponents(chapitre - 1)
                setupButton(scanname, chapitre - 1)
                loadFragment(ReadFragement(this, scanname, chapitre - 1))
            }
        }
    }

    private fun setupCloseButton() {
        val closeButton = findViewById<ImageView>(R.id.close_button)

        closeButton.setOnClickListener {
            val intent = Intent(this, MainActivity::class.java)
            this.startActivity(intent)
        }
    }

    private fun loadFragment(fragment: Fragment) {
        //injecter le fragement
        val transaction = supportFragmentManager.beginTransaction()

        transaction.replace(R.id.fragment_container, fragment)
        transaction.addToBackStack(null)
        transaction.commit()
    }
}