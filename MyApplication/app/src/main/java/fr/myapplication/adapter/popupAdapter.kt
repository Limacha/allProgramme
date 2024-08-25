package fr.myapplication.adapter

import android.view.LayoutInflater
import android.view.View
import android.view.ViewGroup
import android.widget.TextView
import androidx.recyclerview.widget.RecyclerView
import fr.myapplication.MainActivity
import fr.myapplication.R
import fr.myapplication.ScanModel

class popupAdapter(
    private val currentScan: ScanModel
) : RecyclerView.Adapter<popupAdapter.ViewHolder>() {

    class ViewHolder(view: View) : RecyclerView.ViewHolder(view) {
        val chapName: TextView? = view.findViewById(R.id.name_item)
        val chappage: TextView? = view.findViewById(R.id.nb_page_item)
    }

    override fun onCreateViewHolder(parent: ViewGroup, viewType: Int): ViewHolder {
        val view = LayoutInflater
            .from(parent.context)
            .inflate(R.layout.item_vertical_popup_scan, parent, false)
        return popupAdapter.ViewHolder(view)
    }

    override fun getItemCount(): Int = currentScan.nb_episode

    override fun onBindViewHolder(holder: ViewHolder, position: Int) {
        val chapter = currentScan.img_epi[(position + 1).toString()] ?: 0
        holder.chapName?.text = (position + 1).toString()
        holder.chappage?.text = "Chapitre $chapter"
    }


}