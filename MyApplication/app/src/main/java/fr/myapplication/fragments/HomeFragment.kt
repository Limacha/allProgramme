package fr.myapplication.fragments

import android.os.Bundle
import android.view.LayoutInflater
import android.view.View
import android.view.ViewGroup
import androidx.fragment.app.Fragment
import androidx.recyclerview.widget.RecyclerView
import com.google.gson.Gson
import com.google.gson.reflect.TypeToken
import fr.myapplication.MainActivity
import fr.myapplication.R
import fr.myapplication.ScanModel
import fr.myapplication.adapter.ScanAdapter

class HomeFragment(private val context: MainActivity) : Fragment() {

    override fun onCreateView(
        inflater: LayoutInflater,
        container: ViewGroup?,
        savedInstanceState: Bundle?
    ): View? {
        val view = inflater.inflate(R.layout.fragment_home, container, false)

        //creez liste stock scan
        val scanList = arrayListOf<ScanModel>()

        // Ouvre le fichier JSON à partir des assets
        val inputStream = context.assets.open("scan.json")

        // Lit le contenu du fichier dans une chaîne
        val json = inputStream.bufferedReader().use { it.readText() }

        // Analyse le contenu JSON en tant qu'objet Kotlin
        val gson = Gson()
        val type = object : TypeToken<List<ScanModel>>() {}.type
        val data: MutableList<ScanModel> = gson.fromJson(json, type)

        // Ajoute chaque élément à la liste
        scanList.addAll(data)

        //recup recycler view
        val verticalRecyclerView = view?.findViewById<RecyclerView>(R.id.vertical_recycler_view)
        verticalRecyclerView?.adapter = ScanAdapter(context, scanList, R.layout.item_vertical_scan)

        return view
    }
}