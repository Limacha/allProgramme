package fr.myapplication.adapter

import android.content.Intent
import android.net.Uri
import android.view.LayoutInflater
import android.view.View
import android.view.ViewGroup
import android.widget.ImageView
import android.widget.TextView
import androidx.recyclerview.widget.RecyclerView
import com.bumptech.glide.Glide
import fr.myapplication.MainActivity
import fr.myapplication.R
import fr.myapplication.ReadActivity
import fr.myapplication.ScanModel
import fr.myapplication.ScanPopupDownload
import java.io.Serializable

class DownloadAdapter(
    val context: MainActivity,
    private val scanList: List<ScanModel>,
    private val layoutId: Int
) : RecyclerView.Adapter<DownloadAdapter.ViewHolder>() {

    class ViewHolder(view: View) : RecyclerView.ViewHolder(view) {
        val scanImage = view.findViewById<ImageView>(R.id.image_item)
        val scanName: TextView? = view.findViewById(R.id.name_item)
        val scanNb_ep: TextView? = view.findViewById(R.id.nb_episode_item)
        val starIcon: ImageView? = view.findViewById(R.id.start_item)

    }

    override fun onCreateViewHolder(parent: ViewGroup, viewType: Int): DownloadAdapter.ViewHolder {
        val view = LayoutInflater
            .from(parent.context)
            .inflate(layoutId, parent, false)
        return DownloadAdapter.ViewHolder(view)
    }

    override fun getItemCount(): Int = scanList.size

    override fun onBindViewHolder(holder: DownloadAdapter.ViewHolder, position: Int) {
        //recup info scan
        val currentScan = scanList[position]

        //charger image
        Glide.with(context).load(Uri.parse(currentScan.imageFile)).into(holder.scanImage)

        //charge nom
        holder.scanName?.text = currentScan.name

        //charge nombre
        holder.scanNb_ep?.text = currentScan.nb_episode.toString().plus(" ").plus("episode")

        //verif si like
        if (currentScan.liked) {
            holder.starIcon?.setImageResource(R.drawable.ic_star)
        } else {
            holder.starIcon?.setImageResource(R.drawable.ic_unstar)
        }

        holder.itemView.setOnClickListener {
            ScanPopupDownload(this, currentScan, context).show()
        }
    }
}
