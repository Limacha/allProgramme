package fr.myapplication

import android.content.pm.ActivityInfo
import androidx.appcompat.app.AppCompatActivity
import android.os.Bundle
import android.widget.TextView
import androidx.fragment.app.Fragment
import com.google.android.material.bottomnavigation.BottomNavigationView
import fr.myapplication.fragments.CollectionFragement
import fr.myapplication.fragments.DownloadFragment
import fr.myapplication.fragments.HomeFragment

class MainActivity : AppCompatActivity() {


    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        requestedOrientation = ActivityInfo.SCREEN_ORIENTATION_PORTRAIT
        setContentView(R.layout.activity_main)

        val navigationView = findViewById<BottomNavigationView>(R.id.bottomNavigationView)
        navigationView.setOnItemSelectedListener  {
            when (it.itemId) {
                R.id.home_page -> {
                    loadFragment(HomeFragment(this), R.string.home_page_title)
                    return@setOnItemSelectedListener true
                }

                R.id.download_page -> {
                    loadFragment(DownloadFragment(this), R.string.download_page_title)
                    return@setOnItemSelectedListener true
                }

                R.id.liked_page -> {
                    loadFragment(CollectionFragement(this), R.string.liked_page_title)
                    return@setOnItemSelectedListener true
                }

                else -> false
            }
        }

        loadFragment(HomeFragment(this), R.string.home_page_title)
    }

    private fun loadFragment(fragment: Fragment, string: Int) {
        //injecter le fragement
        val transaction = supportFragmentManager.beginTransaction()

        findViewById<TextView>(R.id.page_title).text = resources.getString(string)

        transaction.replace(R.id.fragment_container, fragment)
        transaction.addToBackStack(null)
        transaction.commit()
    }
}